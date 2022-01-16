# Mi Flora

An API that exposes data from Mi Flora devices.
This project is in its infancy, use with caution.

## Tested and supported devices

- [x] Xiaomi Mi Flower Care Plant Sensor
- [ ] Xiaomi Mi Flower Care Plant RoPot

## Devcontainer

The devcontainer currently does not provide bluetooth support and is thus not usable at the moment.

## Contributing

This project uses `cargo deny` to check dependencies.

## Characteristics

### Firmware version & battery level

`00001a02-0000-1000-8000-00805f9b34fb: READ`  
example response: [49, 56, 51, 46, 51, 46, 53]  
18 3.3.5  

### Device mode
`00001a00-0000-1000-8000-00805f9b34fb: READ | WRITE`

### Get real time sensor values
Write `0xA01F` to device mode to get real time sensor values.
`00001a01-0000-1000-8000-00805f9b34fb: READ | WRITE | NOTIFY`

### Get historical sensor values
`00001a11-0000-1000-8000-00805f9b34fb: READ`

### Device time
`00001a12-0000-1000-8000-00805f9b34fb: READ`

### Device Name
`00002a00-0000-1000-8000-00805f9b34fb; READ`
example repsonse [70, 108, 111, 119, 101, 114, 32, 99, 97, 114, 101]
Flower care

<details>
    <summary>Unknown characteristic</summary>

    `00000002-0000-1000-8000-00805f9b34fb`  
    `00000001-0000-1000-8000-00805f9b34fb`  
    `00000004-0000-1000-8000-00805f9b34fb`  
    `00000007-0000-1000-8000-00805f9b34fb`  
    `00000010-0000-1000-8000-00805f9b34fb`  
    `00000013-0000-1000-8000-00805f9b34fb`  
    `00000014-0000-1000-8000-00805f9b34fb`  
    `00001001-0000-1000-8000-00805f9b34fb`  
    `00001a10-0000-1000-8000-00805f9b34fb`  
    `00002a01-0000-1000-8000-00805f9b34fb`  
    `00002a02-0000-1000-8000-00805f9b34fb`  
    `00002a04-0000-1000-8000-00805f9b34fb`  
    `00002a05-0000-1000-8000-00805f9b34fb`  
    `457871e8-d516-4ca1-9116-57d0b17b9cb2`  
    `5f78df94-798c-46f5-990a-b3eb6a065c88`  
    `6c53db25-47a1-45fe-a022-7c92fb334fd4`  
    `6c53db25-47a1-45fe-a022-7c92fb334fd4`  
    `724249f0-5ec3-4b5f-8804-42345af08651`  
    `8082caa8-41a6-4021-91c6-56f9b954cc34`  
    `9d84b9a3-000c-49d8-9183-855b673fda31`  
</details>
