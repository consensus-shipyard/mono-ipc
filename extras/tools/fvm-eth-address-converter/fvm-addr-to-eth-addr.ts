import { ethAddressFromDelegated } from '@glif/filecoin-address'

if (process.argv.length != 3) {
    console.log('Usage: npx ts-node fvm-addr-to-eth-addr.ts <fvm_address>')
    process.exit(1)
}

const fvmAddress = process.argv[2]
const ethAddress: string = ethAddressFromDelegated(fvmAddress)

console.log(ethAddress)
