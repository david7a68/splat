---
splat_schema: 1
splat_id: 8b52c7cd-6dc1-5f02-949a-6df4b15a852c
splat_kind: creature
splat_key: dnd2024:creatures/flesh_golem
splat_name: Flesh Golem
splat_origin:
  key: dnd2024:reference/creature/flesh_golem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 285
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: construct
  alignment: Neutral
  movement_text: 30 ft.
  immunities_text: Lightning, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Understands Common plus one other language but can’t speak
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 127 (15d8 + 60)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 9
  save_dexterity:
    value: -1
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 9
  max_hp:
    value: 127
  initiative:
    value: -1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 5ef0e407664162639662d26defc923487684fe0371d72787ae0cf36eddea1b65
  source_path: builtin/srd-5.2.1/creature/flesh_golem.md
  pages:
  - 285
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Flesh Golem Medium Construct, Neutral AC 9 Initiative -1 (9) HP 127 (15d8 + 60) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 19 +4 +4 Dex 9 -1 -1 Con 18 +4 +4 Int 6 -2 -2 Wis 10 +0 +0 Cha 5 -3
    -3 Immunities Lightning, Poison; Charmed, Exhaustion, Frightened, Paralyzed, Petrified, Poisoned Senses
    Darkvision 60 ft.; Passive Perception 10 Languages Understands Common plus one other language but
    can’t speak CR 5 (XP 1,800; PB +3) ### Traits Aversion to Fire. If the golem takes Fire damage, it
    has Disadvantage on attack rolls and ability checks until the end of its next turn. Berserk. Whenever
    the golem starts its turn Bloodied, roll 1d6. On a 6, the golem goes berserk. On each of its turns
    while berserk, the golem attacks the nearest creature it can see. If no creature is near enough to
    move to and attack, the golem attacks an object. Once the golem goes berserk, it remains so until
    it is destroyed or it is no longer Bloodied. The golem’s creator, if within 60 feet of the berserk
    golem, can try to calm it by taking an action to make a DC 15 Charisma (Persuasion) check; the golem
    must be able to hear its creator. If this check succeeds, the golem ceases being berserk until the
    start of its next turn, at which point it resumes rolling for the Berserk trait again if it is still
    Bloodied. Immutable Form. The golem can’t shape-shift. Lightning Absorption. Whenever the golem is
    subjected to Lightning damage, it regains a number of Hit Points equal to the Lightning damage dealt.
    Magic Resistance. The golem has Advantage on saving throws against spells and other magical effects.
    ### Actions Multiattack. The golem makes two Slam attacks. Slam. Melee Attack Roll: +7, reach 5 ft.
    Hit: 13 (2d8 + 4) Bludgeoning damage plus 4 (1d8) Lightning damage.'
  sha256: 03b6be482d0cbfd0a3e0586bfb8d932bd603cef3b0b33c25effafaacca9685bf
  status: needs_semantic_conversion
---

# Flesh Golem

<!-- SRD page 285 -->

Medium Construct, Neutral\
AC 9 Initiative −1 (9)\
HP 127 (15d8 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 9 −1 −1 Con 18 +4 +4\
Int 6 −2 −2 Wis 10 +0 +0 Cha 5 −3 −3\
Immunities Lightning, Poison; Charmed, Exhaustion,\
Frightened, Paralyzed, Petrified, Poisoned\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Understands Common plus one other language\
but can’t speak\
CR 5 (XP 1,800; PB +3)\

### Traits

Aversion to Fire. If the golem takes Fire damage, it has\
Disadvantage on attack rolls and ability checks until the\
end of its next turn.\
Berserk. Whenever the golem starts its turn Bloodied,\
roll 1d6. On a 6, the golem goes berserk. On each of\
its turns while berserk, the golem attacks the nearest\
creature it can see. If no creature is near enough to\
move to and attack, the golem attacks an object. Once\
the golem goes berserk, it remains so until it is de-\
stroyed or it is no longer Bloodied.\
The golem’s creator, if within 60 feet of the berserk\
golem, can try to calm it by taking an action to make a\
DC 15 Charisma (Persuasion) check; the golem must be\
able to hear its creator. If this check succeeds, the go-\
lem ceases being berserk until the start of its next turn,\
at which point it resumes rolling for the Berserk trait\
again if it is still Bloodied.\
Immutable Form. The golem can’t shape-shift.\
Lightning Absorption. Whenever the golem is sub-\
jected to Lightning damage, it regains a number of Hit\
Points equal to the Lightning damage dealt.\
Magic Resistance. The golem has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The golem makes two Slam attacks.\
Slam. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 +\
4) Bludgeoning damage plus 4 (1d8) Lightning damage.\

Original layout: [[../../srd-5.2.1/pages/285|page 285]].
