// TODO 编写 宏 来收敛重复的代码
macro_rules! read {
    ($service_fn:ident > $vo:ty) => {
        pub async fn $service_fn(
            Path(id): Path<i64>,
            Extension(state): State,
        ) -> AppResult<$vo> {
            let res = state.service.$service_fn(id).await?;

            Resp::ok(res.into())
        }
    };
    ($req_vo:ty > $service_fn:ident > $res_vo:ty) => {
        pub async fn $service_fn(
            Json(params): Json<$req_vo>,
            Extension(state): State,
        ) -> AppResult<$res_vo> {
            let result = state.service.$service_fn(params.into()).await?;
            let vos = result.into_iter().map(|x| x.into()).collect_vec();
            Resp::ok(vos)
        }
    };
    ($req_vo:ty > $service_fn:ident { $in_fn:item $out_fn:item } > $res_vo:ty) => {
        pub async fn $service_fn(
            Json(params): Json<$req_vo>,
            Extension(state): State,
        ) -> AppResult<$res_vo> {
            $in_fn
            $out_fn
            let result = state.service.$service_fn(into(params)).await?;
            // let vos = result.iter().map(|x| UserVO::from(x)).collect_vec();
            Resp::ok(outo(result))
        }
    };
}

macro_rules! create {
    ($req_vo:ident > $service_fn:ident ( $service_input:ident)  > $res_vo:ident) => {
        pub async fn $service_fn(
            Json(params): Json<$req_vo>,
            Extension(state): State,
        ) -> AppResult<$res_vo> {
            let service_input: $service_input = params.into();
            let user = state.service.$service_fn(service_input).await?;

            Resp::ok(user.into())
        }
    };
    (Vec<$req_vo:ident> > $service_fn:ident (Vec<$service_input:ident>)  > Vec<$res_vo:ident>) => {
        pub async fn $service_fn(
            Json(params): Json<Vec<$req_vo>>,
            Extension(state): State,
        ) -> AppResult<Vec<$res_vo>> {
            let service_input: Vec<$service_input> = params.into_iter().map(|x| x.into()).collect();
            let service_result = state.service.$service_fn(service_input).await?;
            let result = service_result.into_iter().map(|x| x.to_string()).collect_vec();
            Resp::ok(result)
        }
    };
}

macro_rules! update {
    ($req_vo:ident -> $service_fn:ident ( $service_input:ident)  -> $res_vo:ident) => {
        pub async fn $service_fn(
            Path(id): Path<i64>,
            Json(mut params): Json<$req_vo>,
            Extension(state): State,
        ) -> AppResult<$res_vo> {
            params.id = Some(id);
            let service_input: $service_input = params.into();
            let user = state.service.$service_fn(service_input).await?;
            Resp::ok(user.into())
        }
    };
}


macro_rules! delete {
    ( $service_fn:ident ) => {
        pub async fn $service_fn(
            Path(ids): Path<String>,
            Extension(state): State
        ) -> impl IntoResponse {
            let ids: Vec<i64> = ids.split(",").into_iter().map(|x| x.trim().parse().unwrap_or(-1)).collect();
            match state.service.$service_fn(ids).await {
                Ok(_) => StatusCode::OK,
                Err(_e) => StatusCode::NOT_FOUND,
            }
        }
    };
}

pub(crate) use read;
pub(crate) use create;
pub(crate) use delete;
pub(crate) use update;