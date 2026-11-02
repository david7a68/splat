---
splat_schema: 1
splat_id: ef91e8bd-92fb-5ef8-89ac-e8c6e74e5002
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_spellcasting
splat_name: Spellcasting
splat_origin:
  key: dnd2024:reference/pages/257
splat_sources:
- title: SRD 5.2.1
  locator: pp. 257
  url: https://www.dndbeyond.com/srd
---

# Spellcasting

## Source text

<!-- SRD page 257 -->

If a monster can cast any spells, its stat block lists the spells and provides the monster’s spellcasting ability, spell save DC (if any spells require a saving throw), and spell attack bonus (if any spells require an attack roll). Unless noted otherwise, a spell of level 1 or higher is always cast at its lowest possible level and can’t be cast at a higher level.

A monster’s spell can have special rules or restrictions. For example, a green hag can cast the Invisibility spell, but the spell has a “self only” restriction, which means the spell affects only the hag.

Spell Components. The Spellcasting trait notes whether the monster’s spellcasting ignores the need for certain spell components. If any spell components are required, describe the monster’s use of Verbal, Somatic, or Material components to signal to characters that it is casting a spell. A monster that requires Material components has them.

Casting Times of 1+ Minutes. If a spell has a casting time of 1 minute or more yet is listed in a spellcasting action, the monster doesn’t cast the spell in just one action unless the action’s description states otherwise; the monster must take the Magic action on each of its turns and maintain Concentration to cast the spell, as described in “Rules Glossary.”

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/parts_of_a_stat_block_spellcasting ` |
| ` implementation ` | ` dnd2024:rule_parts_of_a_stat_block_spellcasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` spellcasting_parameters_source ` | ` monster_stat_block ` |
| ` specific_restrictions_apply ` | ` true ` |
| ` self_only_means ` | ` monster_only ` |

#### Leveled spell default

<!-- splat:values path=splat_rule/parameters/leveled_spell_default -->

| Field | Value |
| --- | --- |
| ` cast_level ` | ` lowest_possible ` |
| ` upcasting ` | ` false ` |

#### Components

<!-- splat:values path=splat_rule/parameters/components -->

| Field | Value |
| --- | --- |
| ` ignore_only_if_trait_says ` | ` true ` |
| ` required_materials_present ` | ` true ` |
| ` required_components_observable ` | ` true ` |

#### Long casting

<!-- splat:values path=splat_rule/parameters/long_casting -->

| Field | Value |
| --- | --- |
| ` minimum_minutes ` | ` 1 ` |
| ` magic_action_each_turn ` | ` true ` |
| ` concentration_required ` | ` true ` |
| ` listing_in_action_does_not_shorten_cast ` | ` true ` |
| ` explicit_action_exception_overrides ` | ` true ` |
