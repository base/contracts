// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockERC20 is ERC20 {
    constructor(string memory name, string memory symbol) ERC20(name, symbol) {}

    function mint(uint256 amount) public virtual {
        _mint(_msgSender(), amount);
    }

    function burn(uint256 amount) public virtual {
        _burn(_msgSender(), amount);
    }
}
