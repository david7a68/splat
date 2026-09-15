---
splat_schema: 1
splat_id: 1cb10e40-4724-5948-8930-5e7b5876854a
splat_kind: creature
splat_key: dnd2024:creatures/giant_fly
splat_name: Giant Fly
splat_origin:
  key: dnd2024:reference/creature/giant_fly
splat_sources:
- title: SRD 5.2.1
  locator: pp. 222
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 30 ft., Fly 60 ft.
  senses_text: Darkvision 60 ft., Passive Perception 10
  languages_text: None
  cr_text: '0 (XP 0; PB +2) Golden Lions (Rare). These gold statuettes of lions are always created in
    pairs. You can use one figurine or both simultaneously. Each can become a Lion for up to 1 hour. Once
    a lion has been used, it can’t be used again until 7 days have passed. Ivory Goats (Rare). These ivory
    statuettes of goats are always created in sets of three. Each goat looks unique and functions differently
    from the others. Their properties are as follows: Goat of Terror. This figurine can become a Giant
    Goat for up to 3 hours. The goat can’t attack, but you can (harmlessly) remove its horns and use them
    as weapons. One horn becomes a +1 Lance, and the other becomes a +2 Longsword. Removing a horn requires
    a Magic action, and the weapons disappear and the horns return when the goat reverts to figurine form.
    While you ride the goat, any Hostile creature that starts its turn within a 30-foot Emanation originating
    from the goat must succeed on a DC 15 Wisdom saving throw or have the Frightened condition for 1 minute,
    until you are no longer riding the goat, or until the goat reverts to figurine form. The Frightened
    creature repeats the save at the end of each of its turns, ending the effect on itself on a success.
    Once it succeeds on the save, a creature is immune to this effect for the next 24 hours. Once the
    figurine has been used, it can’t be used again until 15 days have passed. Goat of Traveling. This
    figurine can become a Large goat with the same statistics as a Riding Horse. It has 24 charges, and
    each hour or portion thereof it spends in goat form costs 1 charge. While it has charges, you can
    use it as often as you wish. When it runs out of charges, it reverts to a figurine and can’t be used
    again until 7 days have passed, when it regains all expended charges. Goat of Travail. This figurine
    can become a Giant Goat for up to 3 hours. Once it has been used, it can’t be used again until 30
    days have passed. Marble Elephant (Rare). This marble statuette resembles a trumpeting elephant. It
    can become an Elephant for up to 24 hours. Once it has been used, it can’t be used again until 7 days
    have passed. Obsidian Steed (Very Rare). This polished obsidian horse can become a Nightmare for up
    to 24 hours. The nightmare fights only to defend itself. Once it has been used, it can’t be used again
    until 5 days have passed. The figurine has a 10 percent chance each time you use it to ignore your
    orders, including a command to revert to figurine form. If you mount the nightmare while it is ignoring
    your orders, you and the nightmare are instantly transported to a random location on the plane of
    Hades, where the nightmare reverts to figurine form. Onyx Dog (Rare). This onyx statuette of a dog
    can become a Mastiff for up to 6 hours. The mastiff has an Intelligence of 8 and can speak Common.
    It also has Blindsight with a range of 60 feet. Once it has been used, it can’t be used again until
    7 days have passed. Serpentine Owl (Rare). This serpentine statuette of an owl can become a Giant
    Owl for up to 8 hours. The owl can communicate telepathically with you at any range if you and it
    are on the same plane of existence. Once it has been used, it can’t be used again until 2 days have
    passed. Silver Raven (Uncommon). This silver statuette of a raven can become a Raven for up to 12
    hours. Once it has been used, it can’t be used again until 2 days have passed. While in raven form,
    the figurine grants you the ability to cast Animal Messenger on it.'
  hit_points_text: 19 (3d10 + 3)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 11
  max_hp:
    value: 19
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_fly
  implementation: dnd2024:creature_giant_fly
  parameters:
    features: {}
    senses:
      darkvision: 60
      passive_perception: 10
    languages:
      speaks: []
      understands: []
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_giant_fly
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.created
  capabilities:
  - creature_statistics
  - attacks
  - damage
  - spatial_queries
  - limited_resources
splat_conversion:
  status: specified
  category: creature
  source_sha256: f75cd007088c137aab508a6a0e9f88c41e59e29f96534d218c07428bd77c3177
  source_path: builtin/srd-5.2.1/creature/giant_fly.md
  pages:
  - 222
  engine_gaps: &id001
  - dnd2024:creature_giant_fly
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 452f7563d3ab8e30fad8d7bbf392df8574c652198bcbf33cd2961404a5e7bff5
splat_clauses:
- key: source_rules
  text: '# Giant Fly Large Beast, Unaligned AC 11 Initiative +1 (11) HP 19 (3d10 + 3) Speed 30 ft., Fly
    60 ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 13 +1 +1 Con 13 +1 +1 Int 2 -4 -4 Wis 10 +0 +0
    Cha 3 -4 -4 Senses Darkvision 60 ft., Passive Perception 10 Languages None CR 0 (XP 0; PB +2) Golden
    Lions (Rare). These gold statuettes of lions are always created in pairs. You can use one figurine
    or both simultaneously. Each can become a Lion for up to 1 hour. Once a lion has been used, it can’t
    be used again until 7 days have passed. Ivory Goats (Rare). These ivory statuettes of goats are always
    created in sets of three. Each goat looks unique and functions differently from the others. Their
    properties are as follows: Goat of Terror. This figurine can become a Giant Goat for up to 3 hours.
    The goat can’t attack, but you can (harmlessly) remove its horns and use them as weapons. One horn
    becomes a +1 Lance, and the other becomes a +2 Longsword. Removing a horn requires a Magic action,
    and the weapons disappear and the horns return when the goat reverts to figurine form. While you ride
    the goat, any Hostile creature that starts its turn within a 30-foot Emanation originating from the
    goat must succeed on a DC 15 Wisdom saving throw or have the Frightened condition for 1 minute, until
    you are no longer riding the goat, or until the goat reverts to figurine form. The Frightened creature
    repeats the save at the end of each of its turns, ending the effect on itself on a success. Once it
    succeeds on the save, a creature is immune to this effect for the next 24 hours. Once the figurine
    has been used, it can’t be used again until 15 days have passed. Goat of Traveling. This figurine
    can become a Large goat with the same statistics as a Riding Horse. It has 24 charges, and each hour
    or portion thereof it spends in goat form costs 1 charge. While it has charges, you can use it as
    often as you wish. When it runs out of charges, it reverts to a figurine and can’t be used again until
    7 days have passed, when it regains all expended charges. Goat of Travail. This figurine can become
    a Giant Goat for up to 3 hours. Once it has been used, it can’t be used again until 30 days have passed.
    Marble Elephant (Rare). This marble statuette resembles a trumpeting elephant. It can become an Elephant
    for up to 24 hours. Once it has been used, it can’t be used again until 7 days have passed. Obsidian
    Steed (Very Rare). This polished obsidian horse can become a Nightmare for up to 24 hours. The nightmare
    fights only to defend itself. Once it has been used, it can’t be used again until 5 days have passed.
    The figurine has a 10 percent chance each time you use it to ignore your orders, including a command
    to revert to figurine form. If you mount the nightmare while it is ignoring your orders, you and the
    nightmare are instantly transported to a random location on the plane of Hades, where the nightmare
    reverts to figurine form. Onyx Dog (Rare). This onyx statuette of a dog can become a Mastiff for up
    to 6 hours. The mastiff has an Intelligence of 8 and can speak Common. It also has Blindsight with
    a range of 60 feet. Once it has been used, it can’t be used again until 7 days have passed. Serpentine
    Owl (Rare). This serpentine statuette of an owl can become a Giant Owl for up to 8 hours. The owl
    can communicate telepathically with you at any range if you and it are on the same plane of existence.
    Once it has been used, it can’t be used again until 2 days have passed. Silver Raven (Uncommon). This
    silver statuette of a raven can become a Raven for up to 12 hours. Once it has been used, it can’t
    be used again until 2 days have passed. While in raven form, the figurine grants you the ability to
    cast Animal Messenger on it.'
  sha256: 6d0a4000b2b89dadbecfaae805065131e592155e9ff5c619c5158b5b45d8bf4a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Fly

<!-- SRD page 222 -->

Large Beast, Unaligned\
AC 11  Initiative +1 (11)\
HP 19 (3d10 + 3)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 14 +2 +2 Dex 13 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 10 +0 +0 Cha 3 −4 −4\
Senses Darkvision 60 ft., Passive Perception 10\
Languages None\
CR 0 (XP 0; PB +2)\
Golden Lions (Rare). These gold statuettes of\
lions are always created in pairs. You can use one\
figurine or both simultaneously. Each can become\
a Lion for up to 1 hour. Once a lion has been used, it\
can’t be used again until 7 days have passed.\
Ivory Goats (Rare). These ivory statuettes of\
goats are always created in sets of three. Each goat\
looks unique and functions differently from the oth-\
ers. Their properties are as follows:\
Goat of Terror. This figurine can become a Giant\
Goat for up to 3 hours. The goat can’t attack, but\
you can (harmlessly) remove its horns and use\
them as weapons. One horn becomes a +1 Lance,\
and the other becomes a +2 Longsword. Removing\
a horn requires a Magic action, and the weapons\
disappear and the horns return when the goat\
reverts to figurine form. While you ride the goat,\
any Hostile creature that starts its turn within\
a 30-foot Emanation originating from the goat\
must succeed on a DC 15 Wisdom saving throw or\
have the Frightened condition for 1 minute, until\
you are no longer riding the goat, or until the goat\
reverts to figurine form. The Frightened creature\
repeats the save at the end of each of its turns,\
ending the effect on itself on a success. Once it\
succeeds on the save, a creature is immune to\
this effect for the next 24 hours. Once the figurine\
has been used, it can’t be used again until 15 days\
have passed.\
Goat of Traveling. This figurine can become a\
Large goat with the same statistics as a Riding\
Horse. It has 24 charges, and each hour or por-\
tion thereof it spends in goat form costs 1 charge.\
While it has charges, you can use it as often as you\
wish. When it runs out of charges, it reverts to a\
figurine and can’t be used again until 7 days have\
passed, when it regains all expended charges.\
Goat of Travail. This figurine can become a Giant\
Goat for up to 3 hours. Once it has been used, it\
can’t be used again until 30 days have passed.\
Marble Elephant (Rare). This marble statuette\
resembles a trumpeting elephant. It can become an\
Elephant for up to 24 hours. Once it has been used,\
it can’t be used again until 7 days have passed.\
Obsidian Steed (Very Rare). This polished ob-\
sidian horse can become a Nightmare for up to 24\
hours. The nightmare fights only to defend itself.\
Once it has been used, it can’t be used again until 5\
days have passed.\
The figurine has a 10 percent chance each time\
you use it to ignore your orders, including a com-\
mand to revert to figurine form. If you mount the\
nightmare while it is ignoring your orders, you\
and the nightmare are instantly transported to a\
random location on the plane of Hades, where the\
nightmare reverts to figurine form.\
Onyx Dog (Rare). This onyx statuette of a dog can\
become a Mastiff for up to 6 hours. The mastiff has\
an Intelligence of 8 and can speak Common. It also\
has Blindsight with a range of 60 feet. Once it has\
been used, it can’t be used again until 7 days have\
passed.\
Serpentine Owl (Rare). This serpentine statuette\
of an owl can become a Giant Owl for up to 8 hours.\
The owl can communicate telepathically with you at\
any range if you and it are on the same plane of ex-\
istence. Once it has been used, it can’t be used again\
until 2 days have passed.\
Silver Raven (Uncommon). This silver statuette\
of a raven can become a Raven for up to 12 hours.\
Once it has been used, it can’t be used again until 2\
days have passed. While in raven form, the figurine\
grants you the ability to cast Animal Messenger on it.\

Original layout: [[../../srd-5.2.1/pages/222|page 222]].
