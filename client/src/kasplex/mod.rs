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

        pub async fn get_indexer_status(&self) -> Result<v1::IndexerStatusResponse> {
            let result =
                get_json::<v1::IndexerStatusResponse>(self.inner.url.join("/info")).await?;
            Ok(result)
        }

        pub async fn get_token_list_page(
            &self,
            cursor: Option<String>,
        ) -> Result<v1::krc20::TokenListResponse> {
            let mut url = self.inner.url.join("/krc20/tokenlist");

            if let Some(cursor) = cursor {
                url = url.join(format!("?next={}", cursor));
            }

            let result = get_json::<v1::krc20::TokenListResponse>(url).await?;
            Ok(result)
        }

        pub async fn get_token_list(&self) -> Result<Vec<v1::krc20::Token>> {
            let mut list = Vec::new();
            let mut cursor = None;
            loop {
                let v1::krc20::TokenListResponse {
                    message: _,
                    next,
                    prev: _,
                    result,
                } = self.get_token_list_page(cursor).await?;
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
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[tokio::test]
        async fn test_get_indexer_status() {
            let indexer = Indexer::try_new(Endpoint::Network {
                network: Network::Testnet11,
                version: 1,
            })
            .unwrap();
            let result = indexer.get_indexer_status().await.unwrap();
            println!("{:?}", result);
        }

        #[tokio::test]
        async fn test_get_token_list() {
            let indexer = Indexer::try_new(Endpoint::Network {
                network: Network::Testnet11,
                version: 1,
            })
            .unwrap();
            let result = indexer.get_token_list().await.unwrap();
            println!("{:?}", result);
        }
    }
}
