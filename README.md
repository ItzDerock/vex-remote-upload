# VEX Wireless Upload
> [!WARNING]
> This project is still in development and is not ready for use.

This project was created for a pretty specific use-case.
The `server` application would run on an SBC (single-board computer) such as a Raspberry Pi Zero W. This SBC would be connected to the VEX v5 Brain and would be mounted somewhere on the robot.

The `cli` application would run on client devices and would communicate to the server to upload code or to view logs. This would allow for the robot to be programmed wirelessly.

## But what about bluetooth
The VEX v5 Cortex + the wireless transmitter allows for Bluetooth communication, but the bandwith is heavily limited. It works fine for uploading code, assuming the code is broken down into a hot/cold module and only the hot module needs to be uploaded.

The main limitation is when it comes to streaming logs. When tuning control algorithms or debugging, it is very useful to have a stream of logs. Sometimes there will be multiple prints per 10ms in a typical loop. This is too much data for the bluetooth connection to handle.

## Using this project
There is no standalone mode, you need to run both the server and client. This is not supposed to be a full replacement of the PROS CLI.

## References
- [vexrs/cargo-v5](https://github.com/vexrs/cargo-v5)