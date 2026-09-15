---
splat_schema: 1
splat_id: f62865d4-766c-54c4-a3a5-db801da34f80
splat_kind: creature
splat_key: dnd2024:creatures/night_hag
splat_name: Night Hag
splat_origin:
  key: dnd2024:reference/creature/night_hag
splat_sources:
- title: SRD 5.2.1
  locator: pp. 311
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Neutral Evil
  movement_text: 30 ft.
  skills_text: Deception +6, Insight +5, Perception +5, Stealth +5
  resistances_text: Cold, Fire
  immunities_text: Charmed
  senses_text: Darkvision 120 ft.; Passive Perception 15
  languages_text: Abyssal, Common, Infernal, Primordial
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 112 (15d8 + 45)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 16
  save_charisma:
    value: 3
  armor_class:
    value: 17
  max_hp:
    value: 112
  initiative:
    value: 5
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_conversion:
  status: partial
  category: creature
  source_sha256: 63acb2a06dfef931ccc143f9fb759e477bf8237b792cedda234e33b3048c7b8b
  source_path: builtin/srd-5.2.1/creature/night_hag.md
  pages:
  - 311
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Night Hag Medium Fiend, Neutral Evil AC 17 Initiative +5 (15) HP 112 (15d8 + 45) Speed 30 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3 Int 16 +3 +3 Wis 14 +2 +2 Cha 16
    +3 +3 Skills Deception +6, Insight +5, Perception +5, Stealth +5 Resistances Cold, Fire Immunities
    Charmed Senses Darkvision 120 ft.; Passive Perception 15 Languages Abyssal, Common, Infernal, Primordial
    CR 5 (XP 1,800; PB +3) ### Traits Coven Magic. While within 30 feet of at least two hag allies, the
    hag can cast one of the following spells, requiring no Material components, using the spell’s normal
    casting time, and using Intelligence as the spellcasting ability (spell save DC 14): Augury, Find
    Familiar, Identify, Locate Object, Scrying, or Unseen Servant. The hag must finish a Long Rest before
    using this trait to cast that spell again. Magic Resistance. The hag has Advantage on saving throws
    against spells and other magical effects. Soul Bag. The hag has a soul bag. While holding or carrying
    the bag, the hag can use its Nightmare Haunting action. The bag has AC 15, HP 20, and Resistance to
    all damage. The bag turns to dust if reduced to 0 Hit Points. If the bag is destroyed, any souls the
    bag is holding are released. The hag can create a new bag after 7 days. ### Actions Multiattack. The
    hag makes two Claw attacks. Claw. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8 + 4) Slashing damage.
    Nightmare Haunting (1/Day; Requires Soul Bag). While on the Ethereal Plane, the hag casts Dream, using
    the same spellcasting ability as Spellcasting. Only the hag can serve as the spell’s messenger, and
    the target must be a creature the hag can see on the Material Plane. The spell fails and is wasted
    if the target is under the effect of the Protection from Evil and Good spell or within a Magic Circle
    spell. If the target takes damage from the Dream spell, the target’s Hit Point maximum decreases by
    an amount equal to that damage. If the spell kills the target, its soul is trapped in the hag’s soul
    bag, and the target can’t be raised from the dead until its soul is released. Spellcasting. The hag
    casts one of the following spells, requiring no Material components and using Intelligence as the
    spellcasting ability (spell save DC 14): At Will: Detect Magic, Etherealness, Magic Missile (level
    4 version) 2/Day Each: Phantasmal Killer, Plane Shift (self only) ### Bonus Actions Shape-Shift. The
    hag shape-shifts into a Small or Medium Humanoid, or it returns to its true form. Other than its size,
    its game statistics are the same in each form. Any equipment it is wearing or carrying isn’t transformed.'
  sha256: b62d6a4363c61f52c40ce1aad71d8ec6d819880e47d44024d2175b675a77fc31
  status: needs_semantic_conversion
---

# Night Hag

<!-- SRD page 311 -->

Medium Fiend, Neutral Evil\
AC 17 Initiative +5 (15)\
HP 112 (15d8 + 45)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 15 +2 +2 Con 16 +3 +3\
Int 16 +3 +3 Wis 14 +2 +2 Cha 16 +3 +3\
Skills Deception +6, Insight +5, Perception +5, Stealth +5\
Resistances Cold, Fire\
Immunities Charmed\
Senses Darkvision 120 ft.; Passive Perception 15\
Languages Abyssal, Common, Infernal, Primordial\
CR 5 (XP 1,800; PB +3)\

### Traits

Coven Magic. While within 30 feet of at least two hag\
allies, the hag can cast one of the following spells,\
requiring no Material components, using the spell’s\
normal casting time, and using Intelligence as the spell-\
casting ability (spell save DC 14): Augury, Find Familiar,\
Identify, Locate Object, Scrying, or Unseen Servant.\
The hag must finish a Long Rest before using this trait to\
cast that spell again.\
Magic Resistance. The hag has Advantage on saving\
throws against spells and other magical effects.\
Soul Bag. The hag has a soul bag. While holding or\
carrying the bag, the hag can use its Nightmare Haunt-\
ing action.\
The bag has AC 15, HP 20, and Resistance to all dam-\
age. The bag turns to dust if reduced to 0 Hit Points. If\
the bag is destroyed, any souls the bag is holding are\
released. The hag can create a new bag after 7 days.\

### Actions

Multiattack. The hag makes two Claw attacks.\
Claw. Melee Attack Roll: +7, reach 5 ft. Hit: 13 (2d8\
+ 4) Slashing damage.\
Nightmare Haunting (1/Day; Requires Soul Bag).\
While on the Ethereal Plane, the hag casts Dream, us-\
ing the same spellcasting ability as Spellcasting. Only\
the hag can serve as the spell’s messenger, and the tar-\
get must be a creature the hag can see on the Material\
Plane. The spell fails and is wasted if the target is under\
the effect of the Protection from Evil and Good spell or\
within a Magic Circle spell.\
If the target takes damage from the Dream spell, the\
target’s Hit Point maximum decreases by an amount\
equal to that damage. If the spell kills the target, its soul\
is trapped in the hag’s soul bag, and the target can’t be\
raised from the dead until its soul is released.\
Spellcasting. The hag casts one of the following spells,\
requiring no Material components and using Intelli-\
gence as the spellcasting ability (spell save DC 14):\
At Will: Detect Magic, Etherealness, Magic Missile\
(level 4 version)\
2/Day Each: Phantasmal Killer, Plane Shift (self only)\

### Bonus Actions

Shape-Shift. The hag shape-shifts into a Small or Me-\
dium Humanoid, or it returns to its true form. Other\
than its size, its game statistics are the same in each\
form. Any equipment it is wearing or carrying isn’t\
transformed.\

Original layout: [[../../srd-5.2.1/pages/311|page 311]].
