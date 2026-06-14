#[macro_export]
macro_rules! impl_mut_player {
(
        // 1. The enum goes at the top now
        $vis:vis enum $enum_name:ident {
            $(
                $(#[$meta:meta])*
                $variant:ident
            ),* $(,)?
        }

        // 2. The required module path
        module = $mod_name:ident;

        // 3. The optional common block goes at the VERY END
        $( common($self_var:ident, $player_var:ident, $options_var:ident) => $common_code:block )?
    ) => {
        // Define the enum
        $vis enum $enum_name {
            $(
                $(#[$meta:meta])*
                $variant,
            )*
        }

        use paste::paste;
        use crate::player::{MutPlayer,BasePlayer,PlayerOptions};
        use crate::options::{HasSelection,FeatureSelections};

        paste! {
            // Implement MutPlayer
            impl MutPlayer for $enum_name {
                fn modify(&self, player: &mut BasePlayer, options: &PlayerOptions) {

                    // Run common code if provided
                    $(
                        {
                            let $self_var = self;
                            let $player_var = &mut *player;
                            let $options_var = options;
                            $common_code
                        }
                    )?

                    match self {
                        $(
                            Self::$variant => {
                                crate::$mod_name::[<$variant:lower>]::modify_player(player, options);
                            }
                        )*
                    }
                }
            }

            // Implement HasSelection
            impl HasSelection for $enum_name {
                fn feature_id(&self) -> usize {
                    match self {
                        $(
                            Self::$variant => crate::$mod_name::[<$variant:lower>]::feature_id(),
                        )*
                    }
                }

                fn selections(&self) -> FeatureSelections {
                    match self {
                        $(
                            Self::$variant => crate::$mod_name::[<$variant:lower>]::selections(),
                        )*
                    }
                }
            }
        }
    };
}


#[macro_export]
macro_rules! define_feature_enum {
    (
        // Allow attributes (like #[derive(...)]) to be passed in
        $(#[$meta:meta])*
        // Capture visibility (pub), the `enum` keyword, and the enum name
        $vis:vis enum $name:ident {
            // Capture the variants
            $(
                $variant:ident
            ),* $(,)?
        }
    ) => {
        // Generate the enum itself
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)] // Added basic derives for usability
        $vis enum $name {
            $( $variant ),*
        }

        // Implement the conversion logic
        impl $name {
            /// Generates the FeatureSelection layout
            pub fn to_feature_selection() -> FeatureSelection {
                use crate::options::FeatureOption;

                FeatureSelection {
                    name: stringify!($name).to_string(),
                    options: vec![
                        $(
                            FeatureOption {
                                name: stringify!($variant).to_string(),
                                // Cast the variant to usize to automatically get a unique ID (0, 1, 2...)
                                id: Self::$variant as usize, 
                            }
                        ),*
                    ],
                }
            }

            /// Reconstructs the enum variant from an ID
            /// Explicitly uses std::option::Option to avoid struct naming conflicts
            pub fn from_id(id: usize) -> std::option::Option<Self> {
                $(
                    if id == Self::$variant as usize {
                        return std::option::Option::Some(Self::$variant);
                    }
                )*
                std::option::Option::None
            }
        }
    };
}