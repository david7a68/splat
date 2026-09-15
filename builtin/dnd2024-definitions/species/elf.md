---
splat_schema: 1
splat_id: ce805c53-e8a6-5937-9b5f-57da4eee5039
splat_kind: rule_group
splat_key: dnd2024:species/elf
splat_name: Elf
splat_origin:
  key: dnd2024:reference/pages/084
splat_sources:
- title: SRD 5.2.1
  locator: pp. 84, 85
  url: https://www.dndbeyond.com/srd
---

# Elf

## Source text

<!-- SRD page 84 -->

Creature Type: Humanoid\
Size: Medium (about 5–6 feet tall)\
Speed: 30 feet\
As an Elf, you have these special traits.\
Darkvision. You have Darkvision with a range of\
60 feet.\
Elven Lineage. You are part of a lineage that\
grants you supernatural abilities. Choose a lineage\
from the Elven Lineages table. You gain the level 1\
benefit of that lineage.\
When you reach character levels 3 and 5, you\
learn a higher-level spell, as shown on the table.\
You always have that spell prepared. You can cast it\
once without a spell slot, and you regain the ability\
to cast it in that way when you finish a Long Rest.\
You can also cast the spell using any spell slots you\
have of the appropriate level.\

<!-- SRD page 85 -->

Elven Lineages\
Lineage Level 1\
Drow The range of your Darkvision increases to 120 feet.\
You also know the Dancing Lights cantrip.\
High Elf You know the Prestidigitation cantrip. Whenever you finish\
a Long Rest, you can replace that cantrip with a different\
cantrip from the Wizard spell list.\
Wood Elf Your Speed increases to 35 feet. You also know the\
Druidcraft cantrip.\
Intelligence, Wisdom, or Charisma is your spell-\
casting ability for the spells you cast with this trait\
(choose the ability when you select the lineage).\
Fey Ancestry. You have Advantage on saving\
throws you make to avoid or end the Charmed\
condition.\
Keen Senses. You have proficiency in the Insight,\
Perception, or Survival skill.\
Trance. You don’t need to sleep, and magic can’t\
put you to sleep. You can finish a Long Rest in 4\
hours if you spend those hours in a trancelike medi-\
tation, during which you retain consciousness.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/elf ` |
| ` implementation ` | ` dnd2024:species_elf ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` size ` | ` medium ` |
| ` speed_feet ` | ` 30 ` |
| ` darkvision_feet ` | ` 60 ` |

#### Height feet approximate

<!-- splat:values path=splat_rule/parameters/height_feet_approximate -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 5 ` |
| ` maximum ` | ` 6 ` |

#### Elven lineage

<!-- splat:yaml path=splat_rule/parameters/elven_lineage -->

```yaml
choice:
  count: 1
  options:
  - id: drow
    level_1:
      darkvision_feet: 120
      cantrip:
        key: dnd2024:spells/dancing_lights
    level_3_spell:
      key: dnd2024:spells/faerie_fire
    level_5_spell:
      key: dnd2024:spells/darkness
  - id: high_elf
    level_1:
      cantrip:
        key: dnd2024:spells/prestidigitation
      replace_cantrip:
        event: rest.long_rest_completed
        list: wizard
        maximum: 1
    level_3_spell:
      key: dnd2024:spells/detect_magic
    level_5_spell:
      key: dnd2024:spells/misty_step
  - id: wood_elf
    level_1:
      speed_feet: 35
      cantrip:
        key: dnd2024:spells/druidcraft
    level_3_spell:
      key: dnd2024:spells/longstrider
    level_5_spell:
      key: dnd2024:spells/pass_without_trace
level_basis: character_level
spellcasting_ability_choice:
  count: 1
  options:
  - intelligence
  - wisdom
  - charisma
leveled_spell_grants:
  always_prepared: true
  each_spell_free_cast:
    spell_slot_cost: 0
    uses:
      maximum: 1
      recover:
        event: rest.long_rest_completed
        amount: all
  can_use_appropriate_spell_slots: true
```

#### Fey ancestry

<!-- splat:yaml path=splat_rule/parameters/fey_ancestry -->

```yaml
saving_throw_advantage:
  avoid_or_end_condition: charmed
```

#### Keen senses

<!-- splat:yaml path=splat_rule/parameters/keen_senses -->

```yaml
skill_proficiency_choice:
  count: 1
  options:
  - insight
  - perception
  - survival
```

#### Trance

<!-- splat:values path=splat_rule/parameters/trance -->

| Field | Value |
| --- | --- |
| ` sleep_required ` | ` false ` |
| ` magical_sleep_immune ` | ` true ` |
| ` long_rest_hours ` | ` 4 ` |
| ` required_activity ` | ` trancelike_meditation ` |
| ` conscious_during_rest ` | ` true ` |
