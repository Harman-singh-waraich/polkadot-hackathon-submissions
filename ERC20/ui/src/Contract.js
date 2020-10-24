import React, { useState} from 'react';
import { ContractPromise } from '@polkadot/api-contract';
import data from "./config/metadata.json";
import { useSubstrate } from './substrate-lib'
import { Form ,Grid} from 'semantic-ui-react';


export default function ContractTransfer(props){
    const contract = Contract();
    const {pair} = props;
    const [to,setTo] = useState()
    const [of,setOf] = useState();
    const [amount,setAmount] = useState(0)
    const [status,setStatus] = useState("");
    const [balance,setBalance] = useState("");
    async function balanceOf(address){
      const bal = await (await contract.read("balance_of",0,3000*10000000,address).send(pair.address)).output.toHuman()
      setBalance(bal);
    }

    async function transfer(){
      setStatus("Transferring...");

      await contract
      .exec('transfer', 0, 3000*100000000,to,amount)
      .signAndSend(pair, (result) => {
      console.log(result);
      setStatus("Transferred :)")

    });    
  
  }
    return(
      <Grid.Row>
      <Grid.Column>
        <h1>Balance Of</h1>
        <Form onSubmit={()=>{balanceOf(of)}}>
        <Form.Group>
          <Form.Input
            placeholder='Address'
            value={of}
            onChange={(e)=>{setOf(e.target.value)}}            
          />
          <Form.Button content='Read' />
        </Form.Group>
        {balance} HAR
      </Form>
  
      </Grid.Column>
      <Grid.Column width={8}>
      <h1>Transfer HAR Tokens</h1>
      <Form onSubmit={()=>{console.log(to,amount),transfer()}}>
        <Form.Group>
          <Form.Input
            placeholder='Address To'
            onChange={(e)=>{setTo(e.target.value)}}
          />
          <Form.Input
            placeholder='Amount'
            onChange={(e)=>{setAmount(e.target.value)}}         

          />
          <Form.Button content='Transfer' />
        </Form.Group>
      </Form>
      {status}
      </Grid.Column>
</Grid.Row>
      )
}

export function Contract(){
  const{api} = useSubstrate();
  return  new ContractPromise(api, data, '5H7zyDNBt2bWfqDJkZ8WrqrjMLkkXoNivL1c1FNkJRfsKEss');
}