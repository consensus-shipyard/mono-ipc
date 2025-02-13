from hashlib import blake2b

# See https://github.com/filecoin-project/FIPs/blob/master/FRCs/frc-0042.md#method-number-computation
def method_number(name):
    name = '1|' + name
    hash = blake2b(name.encode('ascii'), digest_size=64)
    #print('digest: ' + hash.hexdigest())
    #print(f'{len(hash.digest())} bytes long')

    digest = hash.digest()
    while digest:
        chunk = digest[:4]
        num = int.from_bytes(chunk, byteorder='big')
        if num >= 1<<24:
            return num
        digest = digest[4:]
    raise Exception("Method ID could not be determined, please change it") 


# these are all the method names used in the example token actor
methods = ['Name', 'Symbol', 'TotalSupply', 'BalanceOf', 'Allowance', 'IncreaseAllowance',
           'DecreaseAllowance', 'RevokeAllowance', 'Burn', 'TransferFrom', 'Transfer', 'Mint']
for method in methods:
    num = method_number(method)
    #print(f'{num:08x}\t{method}')
    # print out Rust code for use in a test
    print(f'assert_eq!(method_hash!("{method}"), 0x{num:08x});')