#!/bin/bash

mkdir -p certs
step certificate create \
  --profile root-ca \
  "FakeC8y CA" \
  ./certs/ca.crt ./certs/ca.key \
  --no-password --insecure
step certificate create \
  --profile leaf \
  fake-c8y \
  ./certs/c8y.crt ./certs/c8y.key \
  --kty RSA \
  --not-after=8760h \
  --ca ./certs/ca.crt \
  --ca-key ./certs/ca.key \
  --bundle \
  --no-password --insecure
step certificate create \
  --profile leaf \
  iot-device \
  ./certs/device.crt ./certs/device.key \
  --kty RSA \
  --not-after=8760h \
  --ca ./certs/ca.crt \
  --ca-key ./certs/ca.key \
  --bundle \
  --no-password --insecure
