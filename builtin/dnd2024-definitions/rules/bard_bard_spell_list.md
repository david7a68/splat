---
splat_schema: 1
splat_id: 8fb00dd4-55f6-5c26-bbcd-48bc9d403425
splat_kind: rule
splat_key: dnd2024:rules/bard_bard_spell_list
splat_name: Bard Spell List
splat_origin:
  key: dnd2024:reference/pages/033
splat_sources:
- title: SRD 5.2.1
  locator: pp. 33, 34, 35
  url: https://www.dndbeyond.com/srd
---

# Bard Spell List

## Source text

<!-- SRD page 33 -->

This section presents the Bard spell list. The spells\
are organized by spell level and then alphabetized,\
and each spell’s school of magic is listed. In the Spe-\
cial column, C means the spell requires Concentra-\
tion, R means it’s a Ritual, and M means it requires a\
specific Material component.\
Cantrips (Level 0 Bard Spells)\
Spell School Special\
Dancing Lights Illusion C\
Light Evocation —\
Mage Hand Conjuration —\
Mending Transmutation —\
Message Transmutation —\
Minor Illusion Illusion —\
Spell School Special\
Prestidigitation Transmutation —\
Starry Wisp Evocation —\
True Strike Divination —\
Vicious Mockery Enchantment —\
Level 1 Bard Spells\
Spell School Special\
Animal Friendship Enchantment —\
Bane Enchantment C\
Charm Person Enchantment —\
Color Spray Illusion —\
Command Enchantment —\
Comprehend Languages Divination R\
Cure Wounds Abjuration —\
Detect Magic Divination C, R\
Disguise Self Illusion —\
Dissonant Whispers Enchantment —\
Faerie Fire Evocation C\
Feather Fall Transmutation —\
Healing Word Abjuration —\
Heroism Enchantment C\
Hideous Laughter Enchantment C\
Identify Divination R, M\
Illusory Script Illusion R, M\
Longstrider Transmutation —\
Silent Image Illusion C\
Sleep Enchantment C\
Speak with Animals Divination R\
Thunderwave Evocation —\
Unseen Servant Conjuration R\
Level 2 Bard Spells\
Spell School Special\
Aid Abjuration —\
Animal Messenger Enchantment R\
Blindness/Deafness Transmutation —\
Calm Emotions Enchantment C\
Detect Thoughts Divination C\
Enhance Ability Transmutation C\
Enlarge/Reduce Transmutation C\
Enthrall Enchantment C\
Heat Metal Transmutation C\
Hold Person Enchantment C\

<!-- SRD page 34 -->

Spell School Special\
Invisibility Illusion C\
Knock Transmutation —\
Lesser Restoration Abjuration —\
Locate Animals or Plants Divination R\
Locate Object Divination C\
Magic Mouth Illusion R, M\
Mirror Image Illusion —\
See Invisibility Divination —\
Shatter Evocation —\
Silence Illusion C, R\
Suggestion Enchantment C\
Zone of Truth Enchantment —\
Level 3 Bard Spells\
Spell School Special\
Bestow Curse Necromancy C\
Clairvoyance Divination C, M\
Dispel Magic Abjuration —\
Fear Illusion C\
Glyph of Warding Abjuration M\
Hypnotic Pattern Illusion C\
Major Image Illusion C\
Mass Healing Word Abjuration —\
Nondetection Abjuration M\
Plant Growth Transmutation —\
Sending Divination —\
Slow Transmutation C\
Speak with Dead Necromancy —\
Speak with Plants Transmutation —\
Stinking Cloud Conjuration C\
Tiny Hut Evocation R\
Tongues Divination —\
Level 4 Bard Spells\
Spell School Special\
Charm Monster Enchantment —\
Compulsion Enchantment C\
Confusion Enchantment C\
Dimension Door Conjuration —\
Freedom of Movement Abjuration —\
Greater Invisibility Illusion C\
Hallucinatory Terrain Illusion —\
Locate Creature Divination C\
Phantasmal Killer Illusion C\
Polymorph Transmutation C\
Level 5 Bard Spells\
Spell School Special\
Animate Objects Transmutation C\
Awaken Transmutation M\
Dominate Person Enchantment C\
Dream Illusion —\
Geas Enchantment —\
Greater Restoration Abjuration M\
Hold Monster Enchantment C\
Legend Lore Divination M\
Mass Cure Wounds Abjuration —\
Mislead Illusion C\
Modify Memory Enchantment C\
Planar Binding Abjuration M\
Raise Dead Necromancy M\
Scrying Divination C, M\
Seeming Illusion —\
Telepathic Bond Divination R\
Teleportation Circle Conjuration M\
Level 6 Bard Spells\
Spell School Special\
Eyebite Necromancy C\
Find the Path Divination C, M\
Guards and Wards Abjuration M\
Heroes’ Feast Conjuration M\
Irresistible Dance Enchantment C\
Mass Suggestion Enchantment —\
Programmed Illusion Illusion M\
True Seeing Divination M\
Level 7 Bard Spells\
Spell School Special\
Arcane Sword Evocation C, M\
Etherealness Conjuration —\
Forcecage Evocation C, M\
Magnificent Mansion Conjuration M\
Mirage Arcane Illusion —\
Prismatic Spray Evocation —\
Project Image Illusion C, M\
Regenerate Transmutation —\
Resurrection Necromancy M\
Symbol Abjuration M\
Teleport Conjuration —\

<!-- SRD page 35 -->

Level 8 Bard Spells\
Spell School Special\
Antipathy/Sympathy Enchantment —\
Befuddlement Enchantment —\
Dominate Monster Enchantment C\
Glibness Enchantment —\
Mind Blank Abjuration —\
Power Word Stun Enchantment —\
Level 9 Bard Spells\
Spell School Special\
Foresight Divination —\
Power Word Heal Enchantment —\
Power Word Kill Enchantment —\
Prismatic Wall Abjuration —\
True Polymorph Transmutation C\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/bard_bard_spell_list ` |
| ` implementation ` | ` dnd2024:rules_bard_bard_spell_list ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sorting ` | ` ["spell_level", "name"] ` |

#### Class

<!-- splat:values path=splat_rule/parameters/class -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:classes/bard ` |

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | level | school | concentration | ritual | specific_material |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 0 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 1 ` | ` 0 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 2 ` | ` 0 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 3 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 4 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 5 ` | ` 0 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 6 ` | ` 0 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 7 ` | ` 0 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 8 ` | ` 0 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 9 ` | ` 0 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 10 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 11 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 12 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 13 ` | ` 1 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 14 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 15 ` | ` 1 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 16 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 17 ` | ` 1 ` | ` divination ` | ` true ` | ` true ` | ` false ` |
| ` 18 ` | ` 1 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 19 ` | ` 1 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 20 ` | ` 1 ` | ` evocation ` | ` true ` | ` false ` | ` false ` |
| ` 21 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 22 ` | ` 1 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 23 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 24 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 25 ` | ` 1 ` | ` divination ` | ` false ` | ` true ` | ` true ` |
| ` 26 ` | ` 1 ` | ` illusion ` | ` false ` | ` true ` | ` true ` |
| ` 27 ` | ` 1 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 28 ` | ` 1 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 29 ` | ` 1 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 30 ` | ` 1 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 31 ` | ` 1 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 32 ` | ` 1 ` | ` conjuration ` | ` false ` | ` true ` | ` false ` |
| ` 33 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 34 ` | ` 2 ` | ` enchantment ` | ` false ` | ` true ` | ` false ` |
| ` 35 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 36 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 37 ` | ` 2 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 38 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 39 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 40 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 41 ` | ` 2 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 42 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 43 ` | ` 2 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 44 ` | ` 2 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 45 ` | ` 2 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 46 ` | ` 2 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 47 ` | ` 2 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 48 ` | ` 2 ` | ` illusion ` | ` false ` | ` true ` | ` true ` |
| ` 49 ` | ` 2 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 50 ` | ` 2 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 51 ` | ` 2 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 52 ` | ` 2 ` | ` illusion ` | ` true ` | ` true ` | ` false ` |
| ` 53 ` | ` 2 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 54 ` | ` 2 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 55 ` | ` 3 ` | ` necromancy ` | ` true ` | ` false ` | ` false ` |
| ` 56 ` | ` 3 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 57 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 58 ` | ` 3 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 59 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 60 ` | ` 3 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 61 ` | ` 3 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 62 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 63 ` | ` 3 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 64 ` | ` 3 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 65 ` | ` 3 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 66 ` | ` 3 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 67 ` | ` 3 ` | ` necromancy ` | ` false ` | ` false ` | ` false ` |
| ` 68 ` | ` 3 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 69 ` | ` 3 ` | ` conjuration ` | ` true ` | ` false ` | ` false ` |
| ` 70 ` | ` 3 ` | ` evocation ` | ` false ` | ` true ` | ` false ` |
| ` 71 ` | ` 3 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 72 ` | ` 4 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 73 ` | ` 4 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 74 ` | ` 4 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 75 ` | ` 4 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 76 ` | ` 4 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 77 ` | ` 4 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 78 ` | ` 4 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 79 ` | ` 4 ` | ` divination ` | ` true ` | ` false ` | ` false ` |
| ` 80 ` | ` 4 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 81 ` | ` 4 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 82 ` | ` 5 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |
| ` 83 ` | ` 5 ` | ` transmutation ` | ` false ` | ` false ` | ` true ` |
| ` 84 ` | ` 5 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 85 ` | ` 5 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 86 ` | ` 5 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 87 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 88 ` | ` 5 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 89 ` | ` 5 ` | ` divination ` | ` false ` | ` false ` | ` true ` |
| ` 90 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 91 ` | ` 5 ` | ` illusion ` | ` true ` | ` false ` | ` false ` |
| ` 92 ` | ` 5 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 93 ` | ` 5 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 94 ` | ` 5 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 95 ` | ` 5 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 96 ` | ` 5 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 97 ` | ` 5 ` | ` divination ` | ` false ` | ` true ` | ` false ` |
| ` 98 ` | ` 5 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 99 ` | ` 6 ` | ` necromancy ` | ` true ` | ` false ` | ` false ` |
| ` 100 ` | ` 6 ` | ` divination ` | ` true ` | ` false ` | ` true ` |
| ` 101 ` | ` 6 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 102 ` | ` 6 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 103 ` | ` 6 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 104 ` | ` 6 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 105 ` | ` 6 ` | ` illusion ` | ` false ` | ` false ` | ` true ` |
| ` 106 ` | ` 6 ` | ` divination ` | ` false ` | ` false ` | ` true ` |
| ` 107 ` | ` 7 ` | ` evocation ` | ` true ` | ` false ` | ` true ` |
| ` 108 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 109 ` | ` 7 ` | ` evocation ` | ` true ` | ` false ` | ` true ` |
| ` 110 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` true ` |
| ` 111 ` | ` 7 ` | ` illusion ` | ` false ` | ` false ` | ` false ` |
| ` 112 ` | ` 7 ` | ` evocation ` | ` false ` | ` false ` | ` false ` |
| ` 113 ` | ` 7 ` | ` illusion ` | ` true ` | ` false ` | ` true ` |
| ` 114 ` | ` 7 ` | ` transmutation ` | ` false ` | ` false ` | ` false ` |
| ` 115 ` | ` 7 ` | ` necromancy ` | ` false ` | ` false ` | ` true ` |
| ` 116 ` | ` 7 ` | ` abjuration ` | ` false ` | ` false ` | ` true ` |
| ` 117 ` | ` 7 ` | ` conjuration ` | ` false ` | ` false ` | ` false ` |
| ` 118 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 119 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 120 ` | ` 8 ` | ` enchantment ` | ` true ` | ` false ` | ` false ` |
| ` 121 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 122 ` | ` 8 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 123 ` | ` 8 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 124 ` | ` 9 ` | ` divination ` | ` false ` | ` false ` | ` false ` |
| ` 125 ` | ` 9 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 126 ` | ` 9 ` | ` enchantment ` | ` false ` | ` false ` | ` false ` |
| ` 127 ` | ` 9 ` | ` abjuration ` | ` false ` | ` false ` | ` false ` |
| ` 128 ` | ` 9 ` | ` transmutation ` | ` true ` | ` false ` | ` false ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dancing_lights ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/light ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mage_hand ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mending ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/4/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/message ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/5/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/minor_illusion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/6/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/prestidigitation ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/7/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/starry_wisp ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/8/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_strike ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/9/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/vicious_mockery ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/10/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_friendship ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/11/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bane ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/12/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/charm_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/13/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/color_spray ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/14/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/command ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/15/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/comprehend_languages ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/16/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/17/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/18/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/disguise_self ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/19/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dissonant_whispers ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/20/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/faerie_fire ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/21/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/feather_fall ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/22/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/healing_word ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/23/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heroism ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/24/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hideous_laughter ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/25/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/identify ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/26/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/illusory_script ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/27/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/longstrider ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/28/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/silent_image ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/29/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sleep ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/30/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_animals ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/31/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/thunderwave ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/32/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/unseen_servant ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/33/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/aid ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/34/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_messenger ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/35/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/blindness_deafness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/36/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/calm_emotions ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/37/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/detect_thoughts ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/38/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enhance_ability ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/39/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enlarge_reduce ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/40/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/enthrall ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/41/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heat_metal ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/42/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hold_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/43/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/invisibility ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/44/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/knock ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/45/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lesser_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/46/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_animals_or_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/47/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_object ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/48/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magic_mouth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/49/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mirror_image ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/50/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/see_invisibility ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/51/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/shatter ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/52/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/silence ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/53/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/suggestion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/54/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/zone_of_truth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/55/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/bestow_curse ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/56/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/clairvoyance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/57/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dispel_magic ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/58/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fear ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/59/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/glyph_of_warding ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/60/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hypnotic_pattern ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/61/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/major_image ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/62/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_healing_word ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/63/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/nondetection ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/64/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/plant_growth ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/65/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/sending ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/66/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/slow ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/67/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_dead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/68/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_plants ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/69/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/stinking_cloud ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/70/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/tiny_hut ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/71/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/tongues ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/72/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/charm_monster ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/73/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/compulsion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/74/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/confusion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/75/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dimension_door ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/76/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/freedom_of_movement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/77/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/greater_invisibility ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/78/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hallucinatory_terrain ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/79/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/locate_creature ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/80/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/phantasmal_killer ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/81/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/polymorph ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/82/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animate_objects ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/83/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/awaken ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/84/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dominate_person ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/85/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dream ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/86/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/geas ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/87/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/greater_restoration ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/88/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/hold_monster ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/89/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/legend_lore ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/90/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_cure_wounds ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/91/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mislead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/92/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/modify_memory ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/93/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/planar_binding ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/94/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/raise_dead ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/95/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/scrying ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/96/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/seeming ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/97/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/telepathic_bond ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/98/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/teleportation_circle ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/99/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/eyebite ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/100/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/find_the_path ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/101/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/guards_and_wards ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/102/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heroes_feast ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/103/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/irresistible_dance ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/104/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mass_suggestion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/105/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/programmed_illusion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/106/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_seeing ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/107/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/arcane_sword ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/108/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/etherealness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/109/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/forcecage ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/110/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magnificent_mansion ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/111/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mirage_arcane ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/112/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/prismatic_spray ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/113/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/project_image ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/114/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/regenerate ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/115/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/resurrection ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/116/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/symbol ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/117/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/teleport ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/118/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/antipathy_sympathy ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/119/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/befuddlement ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/120/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dominate_monster ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/121/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/glibness ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/122/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/mind_blank ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/123/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/power_word_stun ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/124/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/foresight ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/125/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/power_word_heal ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/126/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/power_word_kill ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/127/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/prismatic_wall ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/128/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/true_polymorph ` |
