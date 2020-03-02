## contract name:
testing network:syntest1

## invoking in javascript

```js
import nearlib from 'nearlib';
const near = nearlib.connect(config);
const wallet = new nearlib.walletAccount(near);
const contract = near.loadContract('syntest1', ....);
```
[near javscript sdk guide](https://docs.nearprotocol.com/docs/roles/developer/examples/nearlib/guides)

## Character Collections Interfaces

Attention:
Return value must be parsed by JSON.parse firstly.
```
get_character({token:123})
```
get detail informations of one character.
- Method Category:View
- Return Type: `{name : string;level : number;
attack:number;defense:number}`
----------------------
```
get_characters_by_owner({owner:'accountName'})
```
get someone's all characters(token).
- Method Category:View
- Return Type:`number[]`
-----------------------------
```
get_balance({owner:'accountName'})
```
get someone's balance.
- Method Category:View
- Return Type:`number`
------------------------------
```
create_random_character({})
```
create a random character for me,return the token of
                      new character.(Cost 10 coins)
- Method Category:Change
- Return Type:`number`
------------------------------
```
level_up_character({token:123})
```
level up my character.(Cost 10 coins)
- Method Category:Change
- Return Type: `void`
--------------------------------
``` 
get_characters_by_owner_detail({owner:'accoutName'})
```
get someone's all characters in detail.
- Method Category:View
- Return Type: `{name : string;level : number;
attack:number;defense:number,
token:number}[]`
