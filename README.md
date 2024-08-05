# WBA-solana-starter
Turbin3 Qohort homework-1 & homework-2

### Case 1 
Although I usually attend the training at 23:00 Turkey time, I completed the rug day task, which was a very useful training without losing anything. 
They divided us into rooms and made us develop together, which was really fun.


#### Commands & Tx Links
I completed the class homework during the session:

Create a mint account:
```bash
ts-node ./cluster1/spl_init.ts
Success! Your mint address is: 2sZ4ZHHTHeB3wSbNvnZeD27FLUrmMx8hzQa18EfyKjKX
```

Create a token account:
```bash
ts-node ./cluster1/spl_mint.ts
Your ata is: BAJaTUTE3eyyeQs7g4eBbhZBPtyG2aYYRX3yycWZnj7Z
Success! Your mint transaction is: 3JSEffd5kXeKTXduzeVVRixNmdfeqEkqruX5LTpv1FqbBkJzyw84gCC96TsBvAU3EPBvCjFkd7tNpuMfkc9fcfjL
```

Mint some tokens:
```bash
Your ata is: BAJaTUTE3eyyeQs7g4eBbhZBPtyG2aYYRX3yycWZnj7Z
Success! Your mint transaction is: 3TUxcfvZFseFyo4JndcprzLtJkPAREvEzoNqnCzX6j3uZ7MLyBgdUDiKRvP7sQsPyxEKatgywtaXJCPrJWvQS2y4
```

### Case 2 
We had Nick Frostbutter doing a presentation (recorded from the morning session) about Blinks.

### Case 3 

#### Commands & Tx Links
- Complete the `spl_transfer` 
```bash
yarn spl_transfer
yarn run v1.22.21
$ ts-node ./cluster1/spl_transfer.ts
Transaction signature: NzCn6uC7oNNWupmm4gLNtujncTEvQG9igodmfB2SK6ah2oz1oWjd7NooyoCPzmJ8ZnEsfeVwtL7siEjsj1CJpaT
✨  Done in 2.93s.
``` 

- Complete the `sql_metadata`

```bash
➜  ts git:(master) ✗ yarn nft_metadata
yarn run v1.22.21
$ ts-node ./cluster1/nft_metadata.ts
Your image URI:  https://arweave.net/IgUpys1136O9Uf-A08Rc20_eP6_FWIAqx4TA3D2_DYI
```

- For Random Rugs
[https://deanmlittle.github.io/generug/](https://deanmlittle.github.io/generug/)


- İnitialized NFT
```bash
yarn nft_mint
yarn run v1.22.21
$ ts-node ./cluster1/nft_mint.ts
Succesfully Minted! Check out your TX here:
https://explorer.solana.com/tx/3Cqz37kLDgzAtihx3gspHMiqvgqXPyT19t5wmWCWydUn8PCJt5N7KemWWB8KaeUcfTfFJotw735oqgJ48XihTDAS?cluster=devnet
Mint Address:  4iDK7p4n3G5C66zhSCcSoVB9nTZfSUy3j88jdZpA8222
✨  Done in 3.28s.
```

[RUG NFT Address](https://explorer.solana.com/address/4iDK7p4n3G5C66zhSCcSoVB9nTZfSUy3j88jdZpA8222?cluster=devnet)
