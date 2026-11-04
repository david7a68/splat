---
splat_schema: 2
splat_id: cb0f259c-303d-568d-8bd2-ead92037ef9a
splat_kind: rule
splat_key: dnd2024:glossary/damage_types
splat_name: Damage Types
splat_origin: dnd2024:reference/glossary/damage_types
splat_sources:
- SRD 5.2.1, pp. 180, https://www.dndbeyond.com/srd
---

# Damage Types

## Source text

<!-- SRD page 180 -->

Attacks and other harmful effects deal different types of damage. Damage types have no rules of their own, but other rules, such as Resistance, rely on the types. The Damage Types table offers examples to help a GM assign a type to a new effect.

**Damage Types**

| Type | Examples |
| --- | --- |
| Acid | Corrosive liquids, digestive enzymes |
| Bludgeoning | Blunt objects, constriction, falling |
| Cold | Freezing water, icy blasts |
| Fire | Flames, unbearable heat |
| Force | Pure magical energy |
| Lightning | Electricity |

| Type | Examples |
| --- | --- |
| Necrotic | Life-draining energy |
| Piercing | Fangs, puncturing objects |
| Poison | Toxic gas, venom |
| Psychic | Mind-rending energy |
| Radiant | Holy energy, searing radiation |
| Slashing | Claws, cutting objects |
| Thunder | Concussive sound |

Original layout: [[../../srd-5.2.1/pages/180|page 180]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/damage_types ` |
| ` implementation ` | ` dnd2024:glossary_damage_types ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` types ` | ` ["acid", "bludgeoning", "cold", "fire", "force", "lightning", "necrotic", "piercing", "poison", "psychic", "radiant", "slashing", "thunder"] ` |
| ` intrinsic_type_rules ` | ` false ` |

#### Gm new effect type

<!-- splat:values path=splat_rule/parameters/gm_new_effect_type -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` options ` | ` ["acid", "bludgeoning", "cold", "fire", "force", "lightning", "necrotic", "piercing", "poison", "psychic", "radiant", "slashing", "thunder"] ` |

#### Example sources

<!-- splat:values path=splat_rule/parameters/example_sources -->

| Field | Value |
| --- | --- |
| ` acid ` | ` ["corrosive_liquid", "digestive_enzyme"] ` |
| ` bludgeoning ` | ` ["blunt_object", "constriction", "falling"] ` |
| ` cold ` | ` ["freezing_water", "icy_blast"] ` |
| ` fire ` | ` ["flame", "unbearable_heat"] ` |
| ` force ` | ` ["pure_magical_energy"] ` |
| ` lightning ` | ` ["electricity"] ` |
| ` necrotic ` | ` ["life_draining_energy"] ` |
| ` piercing ` | ` ["fang", "puncturing_object"] ` |
| ` poison ` | ` ["toxic_gas", "venom"] ` |
| ` psychic ` | ` ["mind_rending_energy"] ` |
| ` radiant ` | ` ["holy_energy", "searing_radiation"] ` |
| ` slashing ` | ` ["claw", "cutting_object"] ` |
| ` thunder ` | ` ["concussive_sound"] ` |
