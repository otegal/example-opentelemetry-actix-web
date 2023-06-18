# example-opentelemetry-actix-web

サンプル

## 構成

### traces
[Actix Web -> OpenTelemetry Collector -> Jaeger]

使い方
```rs
let span = tracing::info_span!("span_name")
span.in_scope(|| {
  // 何か処理を行う。
  // root spanはActix Webのミドルウェアに設定済みのため、エンドポイント配下のspanとして扱える。
});
```
Jaegerでのイメージ
![image](https://github.com/otegal/example-opentelemetry-actix-web/assets/28496166/ec8ee0aa-fe8e-451a-a703-66690c776596)
