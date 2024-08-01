pub mod v1 {

    use crate::imports::*;
    use crate::result::Result;
    use sparkle_core::model::kasplex::v1;

    struct Inner {
        url: Url,
    }

    #[derive(Clone)]
    pub struct Indexer {
        inner: Arc<Inner>,
    }

    impl Indexer {
        pub fn try_new(url: Url) -> Result<Self> {
            Ok(Self {
                inner: Arc::new(Inner { url }),
            })
        }

        pub async fn get_indexer_status(&self) -> Result<v1::IndexerStatus> {
            let response =
                get_json::<v1::IndexerStatusResponse>(self.inner.url.join("/info")).await?;

            if !response.message.starts_with("success") {
                Err(Error::IndexerError(response.message))
            } else {
                Ok(response.result)
            }
        }

        pub async fn get_token_list_page(
            &self,
            cursor: Option<String>,
        ) -> Result<v1::krc20::TokenListResponse> {
            let mut url = self.inner.url.join("/krc20/tokenlist");

            if let Some(cursor) = cursor {
                url = url.join(format!("?next={}", cursor));
            }

            let response = get_json::<v1::krc20::TokenListResponse>(url).await?;
            Ok(response)
        }

        pub async fn get_token_list(&self) -> Result<Vec<v1::krc20::Token>> {
            let mut list = Vec::new();
            let mut cursor = None;
            loop {
                let v1::krc20::TokenListResponse {
                    message,
                    next,
                    prev: _,
                    result,
                } = self.get_token_list_page(cursor).await?;

                if !message.starts_with("success") {
                    return Err(Error::IndexerError(message));
                }

                let len = result.len();
                list.extend(result.into_iter());
                if len < 50 {
                    break;
                } else {
                    cursor = Some(next);
                }
            }

            Ok(list)
        }

        // https://tn11api.kasplex.org/v1/krc20/address/{address}/token/{tick}
        // https://tn11api.kasplex.org/v1/krc20/address/{address}/tokenlist

        pub async fn get_token_balance_list_by_address(
            &self,
            address: &Address,
        ) -> Result<Vec<v1::krc20::TokenBalance>> {
            let url = self
                .inner
                .url
                .join(format!("/krc20/address/{address}/tokenlist"));

            // TODO: loop over paginated results
            let response = get_json::<v1::krc20::TokenBalanceListByAddressResponse>(url).await?;

            if !response.message.starts_with("success") {
                Err(Error::IndexerError(response.message))
            } else {
                Ok(response.result)
            }
        }

        pub async fn get_token_balance_by_address(
            &self,
            address: &Address,
            tick: &str,
        ) -> Result<Vec<v1::krc20::TokenBalance>> {
            let url = self
                .inner
                .url
                .join(format!("/krc20/address/{address}/token/{tick}"));
            let response = get_json::<v1::krc20::TokenBalanceResponse>(url).await?;

            if response.message.starts_with("success") {
                Err(Error::IndexerError(response.message))
            } else {
                Ok(response.result)
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(test)]
    mod test {

        use super::*;

        #[tokio::test]
        async fn test_get_indexer_status() {
            let indexer = Indexer::try_new(v1::Network::Testnet11.into()).unwrap();
            let result = indexer.get_indexer_status().await.unwrap();
            println!("{:?}", result);
        }

        #[tokio::test]
        async fn test_get_token_list() {
            let indexer = Indexer::try_new(v1::Network::Testnet11.into()).unwrap();
            let result = indexer.get_token_list().await.unwrap();
            println!("{:?}", result);
        }
    }
}
