pragma solidity 0.8.0;

contract Test {
    function awesome() public returns (address) {
        // solidhunter-disable-next-line exist-with-error
        return tx.origin; // solidhunter-disable-line exist-with-error
    }

    function awesomeLineUp() public returns (address) {
        // solidhunter-disable-next-line avoid-tx-origin
        return tx.origin;
    }
    function awesomeSameLine() public returns (address) {
        return tx.origin; // solidhunter-disable-line avoid-tx-origin
    }
    
    function notAwesome() public returns (address) {
        // solidhunter-disable-next-line not-exist-no-error
        return msg.sender; // solidhunter-disable-line not-exist-no-error
    }

    function awesomeLineUpAny() public returns (address) {
        // solidhunter-disable-next-line
        return tx.origin;
    }
    function awesomeSameLineAny() public returns (address) {
        return tx.origin; // solidhunter-disable-line
    }
}