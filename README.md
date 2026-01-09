# Rust/Wasm on AWS Lambda@Edge (CloudFront)

**Read the accompanying [blog post][mt-post] to this project.**

> [!IMPORTANT]  
> `2026-01-09` Archived, as I do not plan to maintain the code any further. If you use this for your own setup, keep in mind that dependencies will be outdated and you have to put some work in to get it to a decent state again.

Since we still cannot have native Rust (or WebAssembly for that matter) on AWS Lambda@Edge, we have to come up with our own solution.
Here I present mine: using the AWS Lambda Node.js enviroment to execute our Wasm blob generated from Rust code.

## tl;dr

```sh
make build
make call
make zip
# upload to your AWS Lambda and attach it to a CloudFront distribution (as viewer request trigger)
# check the Cloudwatch logs in the regions you made requests
# be happy - Have a nice day!
```

This demo doesn't do anything fancy. In fact, it does not alter the request at all and only passes it through as-is.

The function will make a few log statements, check the Cloudwatch logs in the regions you received requests for your CloudFront distribution.

This setup is good to test for the baseline performance.

After the cold-start some requests later the function duration should be **around 1 to 2 ms.**

Depending on the region and where you are, this translates to 40 to 50 ms in request latency for a small static asset,
while without any trigger attached the same request would take around 25 ms.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>



<!-- links -->
[mt-post]: https://markentier.tech/posts/2021/01/rust-wasm-on-aws-lambda-edge/
