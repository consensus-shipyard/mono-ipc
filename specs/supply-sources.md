# Supply Sources

## Subnet native coin

Every subnet has a native coin. This is the economic unit that’s passed when sending value in transactions, and it’s used to settle gas payments (both burns and premiums). This coin is externally defined by configuring the “supply source” at the parent, and the subnet itself has no internal knowledge of what the supply is, nor does it need to (thanks to IPC’s decoupled architecture).

## Supply sources

The `SupplySource` of a subnet is configured at subnet creation time. It determines which currency in the parent chain will turn into the native circulating supply of the subnet. The `SupplySource` can’t be modified at the later stage, so make sure you choose it wisely.

It can be one of two kinds (as per the `SupplyKind` enum):

- `Native` (default)
- `ERC20`

The `Native` supply source indicates that the subnet adopts the native coin of the parent as its own native circulating supply currency. `Native` has transitive properties.

- if an L2 subnet is anchored to *Filecoin* as its root chain, the subnet’s currency will be *FIL*.
- if an L3 subnet is anchored to an L2 whose circulating supply is *FIL*, the L3 will adopt *FIL* too (transitive property).

Conversely, the `ERC20` supply source indicates that the subnet adopts an arbitrary ERC20-compliant token residing at the parent as its own native circulating supply currency. Consequently, the total supply of the subnet is controlled by the ERC20 token at the parent. This choice is highly desirable when the developer wants to kickstart a custom cryptoeconomy within their subnet.

When specifying the `ERC20` supply kind, the `SupplySource` must specify `tokenAddress` - the address of the smart contract implementing the ERC20 interface. This token must exist at subnet creation time. Counterfactual token deployments are disallowed.

## Implementation notes

### **Creating subnet with ERC20 `SupplySource`**

In order to create a subnet with non-default `SupplySource` , in the IPC CLI `subnet create` command the subnet owner needs to specify:

- `--supply-source-kind erc20`
- `--supply_source_address <address of ERC20 contract on the parent>`

In order to make sure the specified contract exists `[SupplySourceHelper#validate](https://github.com/consensus-shipyard/ipc/blob/main/contracts/src/lib/SupplySourceHelper.sol#L29)` function is called which executed basic sanity check using `[IERC20#balanceOf](https://github.com/consensus-shipyard/ipc/blob/main/contracts/src/lib/SupplySourceHelper.sol#L38)` which reverts if the contract doesn’t exist or doesn’t implement `balanceOf`.

### Sending funds to the subnet depending on the `SupplySource`

Funds can be sent to the subnet’s receiver address using

- `GatewayManagerFacet#fund(subnetID, to)` if the `Native` supply source is used
- `GatewayManagerFacet#fundWithToken(subnetID, to, amount)` if the `ERC20` supply source is used

`GatewayManagerFacet#fundWithToken` locks a specified amount into custody using `IERC20#safeTransferFrom`.

Both `fund` and `fundWithToken` functions commit a top-down message to be processed by the subnet in order to increase the recipient’s balance.
