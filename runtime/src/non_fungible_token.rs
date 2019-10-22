use support::{decl_module, decl_storage};
//use sr_primitives::traits::Member;
use codec::{Encode, Decode};

pub trait Trait: system::Trait {}

type Uint256 = u32;

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode)]
pub struct PRC721Metadata {
	name: Option<String>,
	symbol: Option<String>,
	tokenUrl: Option<String>,
}

//impl PRC721Metadata {
//    pub fn tokenURI(owner: T::AccountId) -> String {
//        return Self.tokenUrl;
//    }
//    pub fn setTokenURI(tokenId: Uint256, uri: String) {}
//}

decl_storage! {
	trait Store for Module<T: Trait> as PRC721 {
		// Mapping from token ID to owner
        //mapping (Uint256 => address) private _tokenOwner;
		TokenOwner get(owned_token): map (Uint256) => Option<T::AccountId>;

		// Mapping from token ID to approved address
        //mapping (Uint256 => address) private _tokenApprovals;
		TokenApprovals get(approval_token): map(Uint256) => Option<T::AccountId>;

		// Mapping from owner to number of owned token
		//mapping (address => Counters.Counter) private _ownedTokensCount;
		OwnedTokensCount get(owned_token_count): map(T::AccountId) => Uint256;

		// Mapping from owner to operator approvals
		//mapping (address => mapping (address => bool)) private _operatorApprovals;
		OperatorApprovals get(approval_operator): map(T::AccountId) => (T::AccountId, bool);
	}
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
    }
}

impl<T: Trait> Module<T> {
	//event Transfer(from: AccountId, to: AccountId, tokenId: Uint256);
    //event Approval(owner: AccountId, approved: AccountId, tokenId: Uint256);
    //event ApprovalForAll(owner: AccountId, operator: AccountId, approved: bool);

    //pub fn balanceOf(owner: AccountId) -> Uint256 {};
    //pub fn ownerOf(tokenId: Uint256) -> AccountId {};

    //pub fn safeTransferFrom(from: AccountId, to: AccountId, tokenId: Uint256) {};
    //pub fn transferFrom(from: AccountId, to: AccountId, tokenId: Uint256) {};
    //pub fn approve(to: AccountId, tokenId: Uint256) {};
    //pub fn getApproved(tokenId: Uint256) -> AccountId {};

    //pub fn setApprovalForAll(operator: AccountId, _approved: bool) {};
    //pub fn isApprovedForAll(owner: AccountId, operator: AccountId) -> bool {};

    //pub fn safeTransferFrom(from: AccountId, to: AccountId, tokenId: Uint256, data: bytes){};
}
