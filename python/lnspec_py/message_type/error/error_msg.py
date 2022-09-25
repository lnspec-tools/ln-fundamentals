"""
This class is for the Data section of Init Message
as specify in https://github.com/lightning/bolts/blob/master/01-messaging.md#the-error-and-warning-messages
"""

import logging
from typing import List
from lnspec_py.basic_type.int import U16Int
from lnspec_py.basic_type.bitmask import Bitfield
from lnspec_py.basic_type.hex_type import ChannelId
from lnspec_py.message_type.msg import Message


class ErrorMessage(Message):
    """
    The message ERROR is encoded like
    1. [channel_id:channel_id]
    2. [u16:len]
    3. [len*byte:data]
    """

    def __init__(
        self,
        msg_type: U16Int,
        channel_id: ChannelId,
        len: U16Int,
        data: List[int],
    ) -> None:
        self.msg_type = msg_type
        self.name = "error"
        self.channel_id = channel_id
        self.len = len
        self.data = data

    @staticmethod
    def decode(raw_msg: str) -> "ErrorData":
        """
        Decode the ERROR message data from a raw hex message message
        """

        msg_type, raw_msg = U16Int.decode_with_hex_str(raw_msg)
        channel_ID = ChannelId(raw_msg[: 32 * 2])
        raw_msg = raw_msg[32 * 2 :]
        len, raw_msg = U16Int.decode_with_hex_str(raw_msg)
        data = Bitfield.decode(raw_msg[: len.val * 2])
        return ErrorMessage(msg_type, channel_ID, len, data)

    def encode(self) -> str:
        data = ""
        if len(self.data) > 0:
            data = Bitfield.encode(self.data)
            if len(data) < self.len.val * 2:
                data = "0" * (self.len.val * 2 - len(data)) + data
        self.msg_type.encode()
        self.channel_id.encode()
        self.len.encode()
        return (
            self.msg_type.val.hex()
            + self.channel_id.val.hex()
            + self.len.val.hex()
            + data
        )
