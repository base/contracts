// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { Test } from "lib/forge-std/src/Test.sol";

import { Timestamp, GameId, GameType, LibGameId } from "src/libraries/bridge/LibUDT.sol";

contract LibGameId_Pack_Test is Test {
    function testFuzz_pack_roundTrip_succeeds(
        GameType _gameType,
        Timestamp _timestamp,
        address _gameProxy
    )
        public
        pure
    {
        GameId gameId = LibGameId.pack(_gameType, _timestamp, _gameProxy);
        (GameType gameType_, Timestamp timestamp_, address gameProxy_) = LibGameId.unpack(gameId);

        assertEq(gameType_.raw(), _gameType.raw());
        assertEq(timestamp_.raw(), _timestamp.raw());
        assertEq(gameProxy_, _gameProxy);
    }
}
