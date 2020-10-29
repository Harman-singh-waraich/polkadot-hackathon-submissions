pragma solidity ^0.6.6;

contract Auction {
    
   
 uint64 public AuctionId;//To identify an Auction
    
    address payable public  beneficiary;
    uint public aucEnd;
    address public highestBidder;
    uint public highestBid;

    mapping(address => uint) public userBidValue;
    
    
    bool public auctionStatus;

    event NewHighestBid(address bidder, uint amount);
    event AuctionEnded(address winner, uint amount);
    
    constructor(
        uint _biddingTime,
        address payable _beneficiary
    ) public {
        AuctionId = uint64(uint(keccak256(abi.encodePacked(_beneficiary,_biddingTime,now))));
        beneficiary = _beneficiary;
        aucEnd = block.timestamp + _biddingTime;
    }

    function bid() public payable {
        require(now <= aucEnd,"Auction already ended.");
        require(msg.value + userBidValue[msg.sender] > highestBid,"There already is a higher bid. Bid higher!!");
        userBidValue[msg.sender] += msg.value;
        highestBidder = msg.sender;
        highestBid = userBidValue[msg.sender];
        emit NewHighestBid(msg.sender, userBidValue[msg.sender]);
    }

    function withdraw() public returns (bool) {

        require(userBidValue[msg.sender] > 0,"nothing to withdraw");
        require(auctionStatus, "auctionEnd has already been called.");

        uint amount = userBidValue[msg.sender];
        userBidValue[msg.sender] = 0;
            if (!msg.sender.send(amount)) {
                userBidValue[msg.sender] = amount;
                return false;
            }
        
        return true;
    }

    function auctionEnd() public {
        require(block.timestamp >= aucEnd);
        
        auctionStatus = true;

        require(beneficiary.send(highestBid));
        userBidValue[highestBidder] = 0;
        emit AuctionEnded(highestBidder, highestBid);

    }
}