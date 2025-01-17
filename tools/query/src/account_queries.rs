use diem_sdk::{
    rest_client::{diem_api_types::ViewRequest, Client},
    types::{account_address::AccountAddress, validator_config::ValidatorConfig},
};
use libra_types::{
    legacy_types::tower::TowerProofHistoryView,
    move_resource::gas_coin::SlowWalletBalance,
    type_extensions::client_ext::{entry_function_id, ClientExt},
};
/// helper to get libra balance at a SlowWalletBalance type which shows
/// total balance and the unlocked balance.
pub async fn get_account_balance_libra(
    client: &Client,
    account: AccountAddress,
) -> anyhow::Result<SlowWalletBalance> {
    let slow_balance_id = entry_function_id("ol_account", "balance")?;
    let request = ViewRequest {
        function: slow_balance_id,
        type_arguments: vec![],
        arguments: vec![account.to_string().into()],
    };

    let res = client.view(&request, None).await?.into_inner();

    SlowWalletBalance::from_value(res)
}

pub async fn get_tower_state(
    client: &Client,
    account: AccountAddress,
) -> anyhow::Result<TowerProofHistoryView> {
    client
        .get_move_resource::<TowerProofHistoryView>(account)
        .await
}

pub async fn get_val_config(
    client: &Client,
    account: AccountAddress,
) -> anyhow::Result<ValidatorConfig> {
    client.get_move_resource::<ValidatorConfig>(account).await
}
