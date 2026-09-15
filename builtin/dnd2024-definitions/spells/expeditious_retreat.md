---
splat_schema: 1
splat_id: 4dfeb27b-06b6-55e8-af6d-e8fe7a8f2948
splat_kind: spell
splat_key: dnd2024:spells/expeditious_retreat
splat_name: Expeditious Retreat
splat_origin:
  key: dnd2024:reference/spell/expeditious_retreat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 129
  url: https://www.dndbeyond.com/srd
---

# Expeditious Retreat

## Source text

<!-- SRD page 129 -->

Level 1 Transmutation (Sorcerer, Warlock, Wizard)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S\
Duration: Concentration, up to 10 minutes\
You take the Dash action, and until the spell ends,\
you can take that action again as a Bonus Action.\

Original layout: [[../../srd-5.2.1/pages/129|page 129]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 1 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["sorcerer", "warlock", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` true ` |
| ` duration ` | ` Concentration, up to 10 minutes ` |
| ` casting_time ` | ` Bonus Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/expeditious_retreat ` |
| ` implementation ` | ` dnd2024:spell_expeditious_retreat ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` self ` |

#### On cast

<!-- splat:values path=splat_rule/parameters/on_cast -->

| Field | Value |
| --- | --- |
| ` action ` | ` dash ` |
| ` included_in_casting_economy ` | ` true ` |

#### Grant

<!-- splat:values path=splat_rule/parameters/grant -->

| Field | Value |
| --- | --- |
| ` action ` | ` dash ` |
| ` economy ` | ` bonus_action ` |
| ` duration ` | ` spell_duration ` |

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: true
maximum:
  value: 10
  unit: minute
```
