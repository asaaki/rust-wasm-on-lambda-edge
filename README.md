# Rust/WASM on AWS Lambda@Edge (CloudFront)

Since we still cannot have native Rust (or WebAssembly for that matter) on AWS Lambda@Edge, we have to come up with our own solution.
Here I present mine: using the AWS Lambda Node.js enviroment to execute our WASM blob generated from Rust code.

## tl;dr

```sh
make build
make call
make zip
# upload to your AWS Lambda and attach it to a CloudFront distribution (like viewer request)
# check the Cloudwatch logs in the regions you made requests
# be happy - Have a nice day!
```

This demo doesn't do anything fancy. In fact, it does not alter the request at all and only passes it through as-is.
The function will make a few log statements, check the Cloudwatch logs in the regions you received requests for your CloudFront distribution.

This setup is good to test for the baseline performance.

After the cold-start some requests later the function duration should be **around 1 to 2 ms.**

Depending on the region and where you are, this translates to 40 to 50 ms in request latency for a small static asset,
while without any trigger attached the same request would take around 25 ms.

<details>
<summary>Example logs</summary>

```
2021-01-16T17:26:42.783+01:00	START RequestId: 0027415a-2e3b-4ba7-808b-d5b03daedc16 Version: 13

2021-01-16T17:26:42.783+01:00	2021-01-16T16:26:42.759Z undefined INFO (wasm module start)

2021-01-16T17:26:42.836+01:00	2021-01-16T16:26:42.800Z 0027415a-2e3b-4ba7-808b-d5b03daedc16 INFO (wasm handler request call)

2021-01-16T17:26:42.899+01:00	END RequestId: 0027415a-2e3b-4ba7-808b-d5b03daedc16

2021-01-16T17:26:42.899+01:00	REPORT RequestId: 0027415a-2e3b-4ba7-808b-d5b03daedc16 Duration: 100.53 ms Billed Duration: 150 ms Memory Size: 128 MB Max Memory Used: 74 MB Init Duration: 274.48 ms

2021-01-16T17:26:43.116+01:00	START RequestId: 6fd778b0-b7b4-4712-b9bf-b66550c40d37 Version: 13

2021-01-16T17:26:43.119+01:00	2021-01-16T16:26:43.119Z 6fd778b0-b7b4-4712-b9bf-b66550c40d37 INFO (wasm handler request call)

2021-01-16T17:26:43.137+01:00	END RequestId: 6fd778b0-b7b4-4712-b9bf-b66550c40d37

2021-01-16T17:26:43.137+01:00	REPORT RequestId: 6fd778b0-b7b4-4712-b9bf-b66550c40d37 Duration: 17.92 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:43.414+01:00	START RequestId: 6c24900f-8411-4a8b-a0a9-747d2675aeed Version: 13

2021-01-16T17:26:43.497+01:00	2021-01-16T16:26:43.497Z 6c24900f-8411-4a8b-a0a9-747d2675aeed INFO (wasm handler request call)

2021-01-16T17:26:43.578+01:00	END RequestId: 6c24900f-8411-4a8b-a0a9-747d2675aeed

2021-01-16T17:26:43.578+01:00	REPORT RequestId: 6c24900f-8411-4a8b-a0a9-747d2675aeed Duration: 160.52 ms Billed Duration: 200 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:43.768+01:00	START RequestId: 3ca076ee-30d8-477b-b7e1-7d20bbe9c2eb Version: 13

2021-01-16T17:26:43.777+01:00	2021-01-16T16:26:43.777Z 3ca076ee-30d8-477b-b7e1-7d20bbe9c2eb INFO (wasm handler request call)

2021-01-16T17:26:43.798+01:00	END RequestId: 3ca076ee-30d8-477b-b7e1-7d20bbe9c2eb

2021-01-16T17:26:43.798+01:00	REPORT RequestId: 3ca076ee-30d8-477b-b7e1-7d20bbe9c2eb Duration: 26.71 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:44.501+01:00	START RequestId: 4019d859-8ae9-48aa-8643-344d90c571c5 Version: 13

2021-01-16T17:26:44.817+01:00	2021-01-16T16:26:44.537Z 4019d859-8ae9-48aa-8643-344d90c571c5 INFO (wasm handler request call)

2021-01-16T17:26:44.877+01:00	END RequestId: 4019d859-8ae9-48aa-8643-344d90c571c5

2021-01-16T17:26:44.878+01:00	REPORT RequestId: 4019d859-8ae9-48aa-8643-344d90c571c5 Duration: 373.67 ms Billed Duration: 400 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:45.072+01:00	START RequestId: 20914d7c-d5e8-43fe-a836-90ca59658d4e Version: 13

2021-01-16T17:26:45.096+01:00	2021-01-16T16:26:45.077Z 20914d7c-d5e8-43fe-a836-90ca59658d4e INFO (wasm handler request call)

2021-01-16T17:26:45.157+01:00	END RequestId: 20914d7c-d5e8-43fe-a836-90ca59658d4e

2021-01-16T17:26:45.157+01:00	REPORT RequestId: 20914d7c-d5e8-43fe-a836-90ca59658d4e Duration: 82.79 ms Billed Duration: 100 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:45.596+01:00	START RequestId: c60069a7-0977-4bdc-8378-6612968a60f3 Version: 13

2021-01-16T17:26:45.696+01:00	2021-01-16T16:26:45.637Z c60069a7-0977-4bdc-8378-6612968a60f3 INFO (wasm handler request call)

2021-01-16T17:26:45.757+01:00	END RequestId: c60069a7-0977-4bdc-8378-6612968a60f3

2021-01-16T17:26:45.757+01:00	REPORT RequestId: c60069a7-0977-4bdc-8378-6612968a60f3 Duration: 158.08 ms Billed Duration: 200 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:45.955+01:00	START RequestId: 540e86a5-0f05-4e54-b139-e719a9accb84 Version: 13

2021-01-16T17:26:46.016+01:00	2021-01-16T16:26:45.997Z 540e86a5-0f05-4e54-b139-e719a9accb84 INFO (wasm handler request call)

2021-01-16T17:26:46.377+01:00	END RequestId: 540e86a5-0f05-4e54-b139-e719a9accb84

2021-01-16T17:26:46.377+01:00	REPORT RequestId: 540e86a5-0f05-4e54-b139-e719a9accb84 Duration: 420.13 ms Billed Duration: 450 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:46.438+01:00	START RequestId: dab9bd11-81ca-4440-ae51-222ed4ff4449 Version: 13

2021-01-16T17:26:46.456+01:00	2021-01-16T16:26:46.441Z dab9bd11-81ca-4440-ae51-222ed4ff4449 INFO (wasm handler request call)

2021-01-16T17:26:46.478+01:00	END RequestId: dab9bd11-81ca-4440-ae51-222ed4ff4449

2021-01-16T17:26:46.478+01:00	REPORT RequestId: dab9bd11-81ca-4440-ae51-222ed4ff4449 Duration: 36.95 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:46.667+01:00	START RequestId: 1b1bbf77-4c6e-4ad1-b2a1-ac5bb81720d5 Version: 13

2021-01-16T17:26:46.669+01:00	2021-01-16T16:26:46.669Z 1b1bbf77-4c6e-4ad1-b2a1-ac5bb81720d5 INFO (wasm handler request call)

2021-01-16T17:26:46.670+01:00	END RequestId: 1b1bbf77-4c6e-4ad1-b2a1-ac5bb81720d5

2021-01-16T17:26:46.670+01:00	REPORT RequestId: 1b1bbf77-4c6e-4ad1-b2a1-ac5bb81720d5 Duration: 1.40 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:47.195+01:00	START RequestId: 3d110c62-354e-42b5-bacf-1e358386b81a Version: 13

2021-01-16T17:26:47.197+01:00	2021-01-16T16:26:47.197Z 3d110c62-354e-42b5-bacf-1e358386b81a INFO (wasm handler request call)

2021-01-16T17:26:47.198+01:00	END RequestId: 3d110c62-354e-42b5-bacf-1e358386b81a

2021-01-16T17:26:47.198+01:00	REPORT RequestId: 3d110c62-354e-42b5-bacf-1e358386b81a Duration: 1.53 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:47.391+01:00	START RequestId: 2a47c712-c327-49b0-8575-d7f2d090e0b0 Version: 13

2021-01-16T17:26:47.394+01:00	2021-01-16T16:26:47.393Z 2a47c712-c327-49b0-8575-d7f2d090e0b0 INFO (wasm handler request call)

2021-01-16T17:26:47.394+01:00	END RequestId: 2a47c712-c327-49b0-8575-d7f2d090e0b0

2021-01-16T17:26:47.394+01:00	REPORT RequestId: 2a47c712-c327-49b0-8575-d7f2d090e0b0 Duration: 1.50 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:47.852+01:00	START RequestId: fcf12d2b-b293-4436-8472-94d5ed6a0e7b Version: 13

2021-01-16T17:26:47.855+01:00	2021-01-16T16:26:47.855Z fcf12d2b-b293-4436-8472-94d5ed6a0e7b INFO (wasm handler request call)

2021-01-16T17:26:47.856+01:00	END RequestId: fcf12d2b-b293-4436-8472-94d5ed6a0e7b

2021-01-16T17:26:47.856+01:00	REPORT RequestId: fcf12d2b-b293-4436-8472-94d5ed6a0e7b Duration: 1.61 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:48.049+01:00	START RequestId: 7a5b1cde-4861-462b-9fbf-93c08be281c4 Version: 13

2021-01-16T17:26:48.052+01:00	2021-01-16T16:26:48.052Z 7a5b1cde-4861-462b-9fbf-93c08be281c4 INFO (wasm handler request call)

2021-01-16T17:26:48.053+01:00	END RequestId: 7a5b1cde-4861-462b-9fbf-93c08be281c4

2021-01-16T17:26:48.053+01:00	REPORT RequestId: 7a5b1cde-4861-462b-9fbf-93c08be281c4 Duration: 1.50 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:48.503+01:00	START RequestId: 4ea54aca-6195-4f8b-bf8f-02c5bc21068c Version: 13

2021-01-16T17:26:48.506+01:00	2021-01-16T16:26:48.506Z 4ea54aca-6195-4f8b-bf8f-02c5bc21068c INFO (wasm handler request call)

2021-01-16T17:26:48.507+01:00	END RequestId: 4ea54aca-6195-4f8b-bf8f-02c5bc21068c

2021-01-16T17:26:48.507+01:00	REPORT RequestId: 4ea54aca-6195-4f8b-bf8f-02c5bc21068c Duration: 1.58 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB

2021-01-16T17:26:48.733+01:00	START RequestId: 32a60336-8cae-4498-9a25-5380db2d7e76 Version: 13

2021-01-16T17:26:48.736+01:00	2021-01-16T16:26:48.735Z 32a60336-8cae-4498-9a25-5380db2d7e76 INFO (wasm handler request call)

2021-01-16T17:26:48.736+01:00	END RequestId: 32a60336-8cae-4498-9a25-5380db2d7e76

2021-01-16T17:26:48.736+01:00	REPORT RequestId: 32a60336-8cae-4498-9a25-5380db2d7e76 Duration: 1.39 ms Billed Duration: 50 ms Memory Size: 128 MB Max Memory Used: 74 MB
```

</details>

As you can see it takes some time until the node runtime stabilizes, but after that every call will be very fast.
As far as I understand this is a problem of AWS Lambda, not the code.


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
