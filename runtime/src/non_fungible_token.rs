use support::{decl_module, decl_storage};
use rstd::result;
//use sr_primitives::traits::Member;
use codec::{Encode, Decode};

pub trait Trait: system::Trait {}

type Uint256 = u32;

#[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq))]
#[derive(Encode, Decode, Clone, Eq, PartialEq, Debug)]
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

		// Mapping from owner to tokens
		OwnedTokens get(owned_tokens): map (T::AccountId, Option<T::TokenIndex>) => Uint256;

		// Mapping from owner to operator approvals
		//mapping (address => mapping (address => bool)) private _operatorApprovals;
		OperatorApprovals get(approval_operator): map(T::AccountId) => (T::AccountId, bool);
	}
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		pub fn setApprovalForAll(origin, operator: T::AccountId, _approved: bool) -> result::Result {
			let sender = ensure_signed(origin)?;
			ensure!(sender != operator, "can not set approval for owner");
			<OperatorApprovals<T>>::insert(&sender,(&operator, _approved));
			if (!_approved) return Ok(("_approved equel false"))
			let n = Self::owned_token_count(&sender);
			for i in (0..n).iter() {
				let id = Self::owned_tokens((&sender, i));
				<TokenApprovals<T>>::insert(id, &operator);
			} 
			ok(())
		}

		pub fn isApprovedForAll(origin, operator: AccountId) -> bool {
			let sender = ensure_signed(origin)?;
			if (!<OperatorApprovals<T>>::exists(&sender)) {
				return false
			}
			let (ope, isApproved) = Self::approval_operator(&sender);
			if (ope == operator && isApproved == true) {
				return true
			} else {
				return false
			}
		}

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


    //pub fn safeTransferFrom(from: AccountId, to: AccountId, tokenId: Uint256, data: bytes){};
}

