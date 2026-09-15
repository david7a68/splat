---
splat_schema: 1
splat_id: f61b7ff4-6733-5af9-a2db-7e1a86c3f742
splat_kind: rule
splat_key: dnd2024:glossary/stat_block
splat_name: Stat Block
splat_origin:
  key: dnd2024:reference/glossary/stat_block
splat_sources:
- title: SRD 5.2.1
  locator: pp. 188, 189
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:glossary/stat_block
  parameters:
    schema:
      size: size_category
      creature_type: creature_type_and_tags
      alignment:
        default: suggested
        actual_chooser: gm
      armor_class: integer
      initiative:
        modifier: integer
        score: integer
      hit_points:
        static: integer
        dice: dice_expression
        constitution_contribution: integer
      speed: feet_by_mode
      abilities: scores_modifiers_and_save_modifiers
      skills: proficiencies
      resistances: damage_type_list
      vulnerabilities: damage_type_list
      immunities: damage_and_condition_lists
      gear: giveable_or_retrievable_equipment
      senses: special_senses_and_passive_perception
      languages: known_languages
      challenge_rating:
        rating: number
        experience_points: integer
        proficiency_bonus: integer
      traits: always_or_conditionally_active_features
      actions: additional_to_glossary_actions
      bonus_actions: available_bonus_actions
      reactions: available_special_reactions
    magically_created_creature_optional_fields:
    - hit_dice
    - initiative
    - challenge_rating
    attack:
      fields:
      - melee_or_ranged
      - attack_bonus
      - reach_or_range
      - on_hit
      default_target_count: 1
    save_effect:
      fields:
      - save_ability
      - dc
      - eligible_creatures
      - failure_result
      - success_result
    damage:
      mode_choice:
        chooser: gm
        options:
        - static
        - dice
      use_both: false
  implementation: dnd2024:glossary_stat_block
splat_contracts:
- id: dnd2024:glossary_stat_block
  inputs:
    monster:
      type: participant
      required: true
    stat_block:
      type: definition
      required: true
    gm_alignment:
      type: choice
      required: true
    gm_damage_mode:
      type: choice
      required: true
  outputs:
  - monster_statistics
  - attack_defaults
  - damage_mode
  events: []
  capabilities:
  - statistics
  - attacks
  - damage
  - gm_choices
splat_conversion:
  status: specified
  category: glossary
  source_sha256: 8bebfcd9defb3d948fc0aa73f40de4baaf3e9c9a1daac851d14e7f0b060362e1
  source_path: builtin/srd-5.2.1/glossary/stat_block.md
  pages:
  - 188
  - 189
  engine_gaps: &id001
  - dnd2024:glossary_stat_block
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/glossary.md
  specification_sha256: d7cd805b8babef5955d4ccd6384879a85999c7f2f43ca8b3fe0079fbeda01308
splat_clauses:
- key: source_rules
  text: '# Stat Block A stat block contains the game statistics of a monster. Each stat block includes
    the following information presented after the monster’s name. Size. A monster is Tiny, Small, Medium,
    Large, Huge, or Gargantuan. See also “Size.” Creature Type. This entry notes the family of beings
    a monster belongs to, along with any descriptive tags. See also “Creature Type.” Alignment. An alignment
    is suggested for the monster, with the GM determining its actual alignment. See also “Alignment.”
    AC, Initiative, and HP. These entries give the monster’s Armor Class, Initiative, and Hit Points,
    which are detailed in “Playing the Game.” In parentheses after the Hit Points, the monster’s Hit Point
    Dice are provided, along with the contribution of its Constitution, if any, to its Hit Points. Following
    the Initiative modifier is an Initiative score. Some creatures that are created by magic lack Hit
    Dice and Initiative information. Speed. Here the monster’s Speed is provided, along with any special
    speeds. See also “Burrow Speed,” “Climb Speed,” “Fly Speed,” and “Swim Speed.” Ability Scores. A table
    provides the monster’s ability scores, modifiers, and saving throw modifiers, all of which are detailed
    in “Playing the Game.” Skills. This entry lists the monster’s skill proficiencies, if any. See also
    “Playing the Game” (“Proficiency”). Resistances and Vulnerabilities. These entries list the monster’s
    Resistances and Vulnerabilities, if any. See also “Resistance” and “Vulnerability.” Immunities. This
    section lists the monster’s damage and condition Immunities, if any. See also “Immunity.” Gear. If
    the monster has any equipment that can be given away or retrieved, it’s listed in this entry. Senses.
    This entry lists the monster’s special senses, such as Darkvision, and its Passive Perception. See
    also “Passive Perception.” Languages. This entry lists any languages the monster knows. CR. Challenge
    Rating summarizes the threat a monster poses and is detailed in “Monsters.” The Experience Points
    characters receive for defeating a monster and its Proficiency Bonus follow. Some creatures that are
    created by magic have no CR. See also “Challenge Rating” and “Experience Points.” Traits. The monster’s
    traits, if any, are features that are active at all times or in certain situations. Actions. The monster
    can take these actions in addition to those detailed in this glossary. See also “Playing the Game”
    (“Actions”). Bonus Actions. If the monster has Bonus Action options, they are listed in this section.
    Reactions. If the monster can take special Reactions, those are listed in this section. Attack Notation.
    The entry for a monster’s attack starts by identifying whether the attack is a melee or a ranged attack
    and then provides the attack roll’s bonus, its reach or range, and what happens on a hit. An attack
    is against one target unless its entry says otherwise. Saving Throw Effect Notation. If an effect
    forces a saving throw, the effect’s entry starts by identifying the kind of saving throw required
    and then provides the save’s DC, a description of which creatures must make the save, and what happens
    on a failed or a successful save. Damage Notation. A stat block usually provides both a static number
    and a die expression for each instance of damage. For example, an attack might deal 4 (1d4 + 2) damage
    on a hit. The GM determines whether you use the static number or the die expression in parentheses;
    you don’t use both.'
  sha256: 5e8fa00464e3c6ac9869a2f3cdd597706ba000f6261125ac69e29f25cafac46d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Stat Block

<!-- SRD page 188 -->

A stat block contains the game statistics of a mon-\
ster. Each stat block includes the following informa-\
tion presented after the monster’s name.\
Size. A monster is Tiny, Small, Medium, Large,\
Huge, or Gargantuan. See also “Size.”\
Creature Type. This entry notes the family of be-\
ings a monster belongs to, along with any descrip-\
tive tags. See also “Creature Type.”\
Alignment. An alignment is suggested for the\
monster, with the GM determining its actual align-\
ment. See also “Alignment.”\
AC, Initiative, and HP. These entries give the\
monster’s Armor Class, Initiative, and Hit Points,\
which are detailed in “Playing the Game.” In paren-\
theses after the Hit Points, the monster’s Hit Point\
Dice are provided, along with the contribution of its\
Constitution, if any, to its Hit Points. Following the\
Initiative modifier is an Initiative score. Some crea-\
tures that are created by magic lack Hit Dice and\
Initiative information.\
Speed. Here the monster’s Speed is provided,\
along with any special speeds. See also “Burrow\
Speed,” “Climb Speed,” “Fly Speed,” and “Swim\
Speed.”\
Ability Scores. A table provides the monster’s\
ability scores, modifiers, and saving throw modifi-\
ers, all of which are detailed in “Playing the Game.”\
Skills. This entry lists the monster’s skill pro-\
ficiencies, if any. See also “Playing the Game”\
(“Proficiency”).\
Resistances and Vulnerabilities. These entries\
list the monster’s Resistances and Vulnerabilities, if\
any. See also “Resistance” and “Vulnerability.”\
Immunities. This section lists the monster’s\
damage and condition Immunities, if any. See also\
“Immunity.”\
Gear. If the monster has any equipment that can\
be given away or retrieved, it’s listed in this entry.\
Senses. This entry lists the monster’s special\
senses, such as Darkvision, and its Passive Percep-\
tion. See also “Passive Perception.”\
Languages. This entry lists any languages the\
monster knows.\

<!-- SRD page 189 -->

CR. Challenge Rating summarizes the threat a\
monster poses and is detailed in “Monsters.” The\
Experience Points characters receive for defeating\
a monster and its Proficiency Bonus follow. Some\
creatures that are created by magic have no CR. See\
also “Challenge Rating” and “Experience Points.”\
Traits. The monster’s traits, if any, are features\
that are active at all times or in certain situations.\
Actions. The monster can take these actions in\
addition to those detailed in this glossary. See also\
“Playing the Game” (“Actions”).\
Bonus Actions. If the monster has Bonus Action\
options, they are listed in this section.\
Reactions. If the monster can take special Reac-\
tions, those are listed in this section.\
Attack Notation. The entry for a monster’s attack\
starts by identifying whether the attack is a melee\
or a ranged attack and then provides the attack\
roll’s bonus, its reach or range, and what happens\
on a hit. An attack is against one target unless its\
entry says otherwise.\
Saving Throw Effect Notation. If an effect forces\
a saving throw, the effect’s entry starts by identify-\
ing the kind of saving throw required and then pro-\
vides the save’s DC, a description of which creatures\
must make the save, and what happens on a failed\
or a successful save.\
Damage Notation. A stat block usually provides\
both a static number and a die expression for each\
instance of damage. For example, an attack might\
deal 4 (1d4 + 2) damage on a hit. The GM deter-\
mines whether you use the static number or the die\
expression in parentheses; you don’t use both.\

Original layout: [[../../srd-5.2.1/pages/188|page 188]], [[../../srd-5.2.1/pages/189|page 189]].
