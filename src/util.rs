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
