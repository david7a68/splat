---
splat_schema: 2
splat_id: 6f444684-28e3-5892-aeae-b3686a61aa0f
splat_kind: rule
splat_key: dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity
splat_name: 'Level 6: Elemental Affinity'
splat_origin: dnd2024:reference/pages/070
splat_sources:
- SRD 5.2.1, pp. 70, https://www.dndbeyond.com/srd
---

# Level 6: Elemental Affinity

## Source text

<!-- SRD page 70 -->

Your draconic magic has an affinity with a damage type associated with dragons. Choose one of those types: Acid, Cold, Fire, Lightning, or Poison.

You have Resistance to that damage type, and when you cast a spell that deals damage of that type, you can add your Charisma modifier to one damage roll of that spell.

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/draconic_sorcery_level_6_elemental_affinity ` |
| ` implementation ` | ` dnd2024:class_features_draconic_sorcery_level_6_elemental_affinity ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` resistance ` | ` chosen_damage_type ` |

#### Damage type choice

<!-- splat:values path=splat_rule/parameters/damage_type_choice -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` options ` | ` ["acid", "cold", "fire", "lightning", "poison"] ` |

#### Spell damage bonus

<!-- splat:yaml path=splat_rule/parameters/spell_damage_bonus -->

```yaml
requires_spell_deals_chosen_type: true
maximum_rolls_per_spell: 1
bonus:
  stat: abilities.charisma.modifier
optional: true
```
