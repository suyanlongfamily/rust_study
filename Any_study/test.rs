       
        fn deal_other_req(&mut self, rpc:&mut RpcRequest) -> cita_response::RpcResult{ 

            let proto_req: Result<request::Request, RpcError> = rpc.clone().try_into();
            if let (code,true) = is_valid_param(proto_req) {
                let req = proto_req.unwrap();
                //发送给mq,等待返回值,并返回结果
                let (code,rece_msg) = send_mq(get_topic(rpc.method),req);

            }{
                (code,"")
            }
        }

            // fn writ_respone() -> String;

        fn send_mq(&self,rpc,topic:String) ->(ErrorCode,communication::Message){ 
            
            if 
        
            //向mq发送数据。
            self.tx.lock().unwrap().send((topic, req.into())).unwrap();
            let recv_msg;
            trace!("wait response {:?}", req_key);
            let mut timeout_count = 0;
            loop {
                timeout_count = timeout_count + 1;
                if timeout_count > self.timeout_count {
                    (ErrorCode::TimeOut,"")
                }
                thread::sleep(Duration::new(0, self.sleep_duration * 1000000));
                if responses.read().unwrap().contains_key(&req_key) {
                    let mut responses = responses.write().unwrap();
                    if let Some(res) = responses.remove(&req_key) {
                        recv_msg = res.result.unwrap();
                    } else {
                        (ErrorCode::DoubleTx,"")
                    }
                    break;
                }
            }

            let mut rpc_response = cita_response::RpcResult {
                id: rpc.id.clone(),
                jsonrpc: rpc.jsonrpc.clone(),
                result: cita_response::ResponseBody::Null,
                error: ResErrBody::default(),
            };
        
            rpc_response.result = cita_response::ResponseBody::from(recv_msg);
            (ErrorCode::Success,serde_json::to_string(&ResErr::from(rpc_response)).unwrap().as_bytes())) 
        }



        fn is_valid_param<T>(&self,req_rpc:&T) -> bool {
            match req_rpc{
                Err(RpcError::NotFound) => {
                    ("-32601",false)
                }
                Err(RpcError::InvalidParams) => {
                    ("-32602",false)
                }
                Ok(_) => ("00000",true)
            }
        }




    //-------------test--------------
    let (code,msg) = match check_url(path){
        Err(err) => (,),
        Ok(_) => {
            let rpc: Result<RpcRequest, serde_json::Error> = serde_json::from_str(&body);
            match rpc {
                Err(err) => (,),
                Ok(rpc) => {
                    let topic = self.get_topic(rpc.mathod);
                    info!("-----rpc dispacth topic: {:?}-----", topic);
                    match rpc.req_type(){
                        ReqType::TX => {
                            deal_tx();

                        },
                        ReqType::OTHER =>{
                            deal_other_req();

                        },
                    },
                },
            }
        }
    }
//--------------test-----------




//同一个Rpc请求。可以对ws、http有不同的行为。
impl Rpc {
    //不同有不同的实现。
        fn check_uri(&self,body:String,) -> Result<>{
            let rpc: Result<RpcRequest, serde_json::Error> = serde_json::from_str(&body);
            match rpc {
                Ok(res) => res,
                Err(err) =>
            }
            if rpc.is_err() {
                trace!("INVALID_REQUEST: body: {:?}", body);
                try_return!(res.send(INVALID_REQUEST.as_bytes()));
                return;
            }

        }

    fn get_topic(&self, method:&String) -> String {
        let topic = if method.starts_with("cita_send") {
                "jsonrpc.new_tx"
            } else if method.starts_with("cita") {
                "jsonrpc.request"
            } else if method.starts_with("net_") {
                "jsonrpc.net"
            } else {
                "jsonrpc"
            }
            .to_string();
        topic
    }

    fn route(&self) {


    }

    fn dispacth_topic(&self) {

    }

    fn do_timeout(&self){

    }
}




fn select_code(code:ErrorCode) ->String {


}



impl Handler for RpcHandler {
    fn handle(&self, req: Request, res: Response) {

        let uri = req.uri.clone();
        let method = req.method.clone();
        let body = read_to_string(req).unwrap();
        trace!("receive Request method: {:?}, body: {:?}", method, body);

        match uri {

            AbsolutePath(ref path) => {
                match (&method, &path[..]) {
                    (&Post, "/") => {}
                    _ => {
                        try_return!(res.send(INVALID_REQUEST.as_bytes()));
                        return;
                    }
                }
            }
            _ => {
                return;
            }
        }

        let rpc: Result<RpcRequest, serde_json::Error> = serde_json::from_str(&body);
        if rpc.is_err() {
            trace!("INVALID_REQUEST: body: {:?}", body);
            try_return!(res.send(INVALID_REQUEST.as_bytes()));
            return;
        }

        let rpc = rpc.unwrap();
        let mut rpc_response = cita_response::RpcResult {
            id: rpc.id.clone(),
            jsonrpc: rpc.jsonrpc.clone(),
            result: cita_response::ResponseBody::Null,
            error: ResErrBody::default(),
        };

        //得到topic
        let topic = get_topic(rpc.method);
        info!("-----rpc dispacth topic: {:?}-----", topic);



        let proto_req: Result<request::Request, RpcError> = rpc.clone().try_into();
        match proto_req {
            Err(RpcError::NotFound) => {}
            Err(RpcError::InvalidParams) => {
                rpc_response.error = ResErrBody {
                    code: "-32602".to_string(),
                    message: "Invalid params".to_string(),
                };
                trace!("InvalidParams: rpc: {:?}", rpc);
                try_return!(res.send(serde_json::to_string(&ResErr::from(rpc_response))
                                         .unwrap()
                                         .as_bytes()));
                return;
            }
            Ok(req) => {
                let request_id: Vec<u8> = req.request_id.clone();
                self.tx
                    .lock()
                    .unwrap()
                    .send((topic, req.into()))
                    .unwrap();

                let recv_msg;
                trace!("wait response {:?}", request_id);
                let mut timeout_count = 0;
                loop {
                    timeout_count = timeout_count + 1;
                    if timeout_count > self.timeout_count {
                        rpc_response.error = ResErrBody {
                            code: "-32602".to_string(),
                            message: "timeout".to_string(),
                        };
                        try_return!(res.send(serde_json::to_string(&ResErr::from(rpc_response))
                                                 .unwrap()
                                                 .as_bytes()));
                        return;
                    }
                    thread::sleep(Duration::new(0, self.sleep_duration * 1000000));
                    if self.responses
                           .read()
                           .unwrap()
                           .contains_key(&request_id) {
                        let mut responses = self.responses.write().unwrap();
                        if let Some(res) = responses.remove(&request_id) {
                            recv_msg = res.result.unwrap();
                        } else {
                            rpc_response.error = ResErrBody {
                                code: "-32602".to_string(),
                                message: "Duplicated transaction".to_string(),
                            };
                            try_return!(
                                res.send(
                                    serde_json::to_string(&ResErr::from(rpc_response))
                                        .unwrap()
                                        .as_bytes()
                                )
                            );
                            return;
                        }
                        break;
                    }
                }
                rpc_response.result = cita_response::ResponseBody::from(recv_msg);
                try_return!(res.send(serde_json::to_string(&RpcResponse::from(rpc_response))
                                         .unwrap()
                                         .as_bytes()));



                rpc_response.result = cita_response::ResponseBody::from(recv_msg);
                try_return!(res.send(serde_json::to_string(&RpcResponse::from(rpc_response))
                                         .unwrap()
                                         .as_bytes()));

                return;
            }
        }




        let proto_ts: Result<blockchain::Transaction, RpcError> = rpc.try_into();
        match proto_ts {
            Err(RpcError::NotFound) => {
                rpc_response.error = ResErrBody {
                    code: "-32601".to_string(),
                    message: "Method not found".to_string(),
                };
                try_return!(res.send(serde_json::to_string(&ResErr::from(rpc_response))
                                         .unwrap()
                                         .as_bytes()));
                return;
            }
            Err(RpcError::InvalidParams) => {
                rpc_response.error = ResErrBody {
                    code: "-32602".to_string(),
                    message: "Invalid params".to_string(),
                };
                try_return!(res.send(serde_json::to_string(&ResErr::from(rpc_response))
                                         .unwrap()
                                         .as_bytes()));
                return;
            }
            Ok(ts) => {
                // transaction auth check
                let conn = self.db_pool.get();
                if conn.is_err() {
                    rpc_response.error = ResErrBody {
                        code: "-32603".to_string(),
                        message: "DB Error".to_string(),
                    };
                    try_return!(res.send(serde_json::to_string(&ResErr::from(rpc_response))
                                             .unwrap()
                                             .as_bytes()));
                    return;
                }

                let conn = conn.unwrap();
                let cita_tx = CitaTransaction::new(ts.clone(), conn.deref());
                trace!("cita_tx: {:?}", cita_tx);
                if !cita_tx.is_valid() {
                    rpc_response.error = ResErrBody {
                        code: "-32604".to_string(),
                        message: "Auth Error".to_string(),
                    };
                    try_return!(res.send(serde_json::to_string(&ResErr::from(rpc_response))
                                             .unwrap()
                                             .as_bytes()));
                    return;
                };
                let hash = ts.sha3();

                self.tx
                    .lock()
                    .unwrap()
                    .send((topic, ts.into()))
                    .unwrap();

                let recv_msg;
                trace!("wait response {:?}", hash);
                let mut timeout_count = 0;
                loop {
                    timeout_count = timeout_count + 1;
                    if timeout_count > self.timeout_count {
                        rpc_response.error = ResErrBody {
                            code: "-32602".to_string(),
                            message: "timeout".to_string(),
                        };
                        try_return!(res.send(serde_json::to_string(&ResErr::from(rpc_response))
                                                 .unwrap()
                                                 .as_bytes()));
                        return;
                    }
                    thread::sleep(Duration::new(0, self.sleep_duration * 1000000));
                    if self.tx_responses.read().unwrap().contains_key(&hash) {
                        trace!("get response");
                        let mut tx_responses = self.tx_responses.write().unwrap();
                        if let Some(msg) = tx_responses.remove(&hash) {
                            recv_msg = msg;
                        } else {
                            rpc_response.error = ResErrBody {
                                code: "-32602".to_string(),
                                message: "Duplicated transaction".to_string(),
                            };
                            try_return!(
                                res.send(
                                    serde_json::to_string(&ResErr::from(rpc_response))
                                        .unwrap()
                                        .as_bytes()
                                )
                            );
                            return;
                        }
                        break;
                    }
                }

                rpc_response.result = cita_response::ResponseBody::from(recv_msg);

                try_return!(res.send(serde_json::to_string(&RpcResponse::from(rpc_response))
                                         .unwrap()
                                         .as_bytes()));
                return;
            }
        }
    }
}
