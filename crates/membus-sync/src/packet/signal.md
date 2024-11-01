Signal packet

This packet defined the layout of signal packet

For more description of [`Signal`], please go and see it's document

# Layout

| signal |      payload_size      |      payload      |
| :----: | :--------------------: | :---------------: |
|  8bit  | 1usize(64bit normally) | 1payload_size bit |
