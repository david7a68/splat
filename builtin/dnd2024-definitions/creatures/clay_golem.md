---
splat_schema: 1
splat_id: 4dfa9536-a6f6-54e9-88fb-c0bf824be2f3
splat_kind: creature
splat_key: dnd2024:creatures/clay_golem
splat_name: Clay Golem
splat_origin:
  key: dnd2024:reference/creature/clay_golem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 274
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: construct
  alignment: Unaligned
  movement_text: 30 ft.
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Acid, Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 9
  languages_text: Common plus one other language
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 123 (13d10 + 52)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 8
  save_wisdom:
    value: -1
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 14
  max_hp:
    value: 123
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 1eb7d793f44a2c2fd2e618f2be0d41bf5242823ae47defb38a929a0519c746d9
  source_path: builtin/srd-5.2.1/creature/clay_golem.md
  pages:
  - 274
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Clay Golem Large Construct, Unaligned AC 14 Initiative +3 (13) HP 123 (13d10 + 52) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +5 Dex 9 -1 -1 Con 18 +4 +4 Int 3 -4 -4 Wis 8 -1 -1 Cha 1
    -5 -5 Resistances Bludgeoning, Piercing, Slashing Immunities Acid, Poison, Psychic; Charmed, Exhaustion,
    Frightened, Paralyzed, Petrified, Poisoned Senses Darkvision 60 ft.; Passive Perception 9 Languages
    Common plus one other language CR 9 (XP 5,000; PB +4) ### Traits Acid Absorption. Whenever the golem
    is subjected to Acid damage, it takes no damage and instead regains a number of Hit Points equal to
    the Acid damage dealt. Berserk. Whenever the golem starts its turn Bloodied, roll 1d6. On a 6, the
    golem goes berserk. On each of its turns while berserk, the golem attacks the nearest creature it
    can see. If no creature is near enough to move to and attack, the golem attacks an object. Once the
    golem goes berserk, it continues to be berserk until it is destroyed or it is no longer Bloodied.
    Immutable Form. The golem can’t shape-shift. Magic Resistance. The golem has Advantage on saving throws
    against spells and other magical effects. ### Actions Multiattack. The golem makes two Slam attacks,
    or it makes three Slam attacks if it used Hasten this turn. Slam. Melee Attack Roll: +9, reach 5 ft.
    Hit: 10 (1d10 + 5) Bludgeoning damage plus 6 (1d12) Acid damage, and the target’s Hit Point maximum
    decreases by an amount equal to the Acid damage taken. ### Bonus Actions Hasten (Recharge 5–6). The
    golem takes the Dash and Disengage actions.'
  sha256: 7175e72bfb8b5393429379b81c69561117d7441560c24466d5f1bb51c04e05aa
  status: needs_semantic_conversion
---

# Clay Golem

<!-- SRD page 274 -->

Large Construct, Unaligned\
AC 14 Initiative +3 (13)\
HP 123 (13d10 + 52)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 9 −1 −1 Con 18 +4 +4\
Int 3 −4 −4 Wis 8 −1 −1 Cha 1 −5 −5\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Acid, Poison, Psychic; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 9\
Languages Common plus one other language\
CR 9 (XP 5,000; PB +4)\

### Traits

Acid Absorption. Whenever the golem is subjected to\
Acid damage, it takes no damage and instead regains a\
number of Hit Points equal to the Acid damage dealt.\
Berserk. Whenever the golem starts its turn Bloodied,\
roll 1d6. On a 6, the golem goes berserk. On each of\
its turns while berserk, the golem attacks the nearest\
creature it can see. If no creature is near enough to\
move to and attack, the golem attacks an object. Once\
the golem goes berserk, it continues to be berserk until\
it is destroyed or it is no longer Bloodied.\
Immutable Form. The golem can’t shape-shift.\
Magic Resistance. The golem has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The golem makes two Slam attacks, or it\
makes three Slam attacks if it used Hasten this turn.\
Slam. Melee Attack Roll: +9, reach 5 ft. Hit: 10 (1d10\
+ 5) Bludgeoning damage plus 6 (1d12) Acid damage,\
and the target’s Hit Point maximum decreases by an\
amount equal to the Acid damage taken.\

### Bonus Actions

Hasten (Recharge 5–6). The golem takes the Dash and\
Disengage actions.\

Original layout: [[../../srd-5.2.1/pages/274|page 274]].
