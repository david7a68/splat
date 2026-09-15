---
splat_schema: 1
splat_id: cd5c201b-e2f5-53a7-8358-133b1488bffa
splat_kind: spell
splat_key: dnd2024:spells/heroes_feast
splat_name: Heroes’ Feast
splat_origin:
  key: dnd2024:reference/spell/heroes_feast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140
  url: https://www.dndbeyond.com/srd
---

# Heroes’ Feast

## Source text

<!-- SRD page 140 -->

Level 6 Conjuration (Bard, Cleric, Druid)\
Casting Time: 10 minutes\
Range: Self\
Components: V, S, M (a gem-encrusted bowl worth\
1,000+ GP, which the spell consumes)\
Duration: Instantaneous\
You conjure a feast that appears on a surface in\
an unoccupied 10-foot Cube next to you. The feast\
takes 1 hour to consume and disappears at the end\
of that time, and the beneficial effects don’t set in\
until this hour is over. Up to twelve creatures can\
partake of the feast.\
A creature that partakes gains several benefits,\
which last for 24 hours. The creature has Resis-\
tance to Poison damage, and it has Immunity to the\
Frightened and Poisoned conditions. Its Hit Point\
maximum also increases by 2d10, and it gains the\
same number of Hit Points.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 6 ` |
| ` school ` | ` conjuration ` |
| ` classes ` | ` ["bard", "cleric", "druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 10 minutes ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` self ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` a gem-encrusted bowl worth 1,000+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/heroes_feast ` |
| ` implementation ` | ` dnd2024:spell_heroes_feast ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` benefits_start ` | ` consumption_hour_completed ` |

#### Spell lifetime

<!-- splat:values path=splat_rule/parameters/spell_lifetime -->

| Field | Value |
| --- | --- |
| ` concentration ` | ` false ` |
| ` instantaneous ` | ` true ` |

#### Feast

<!-- splat:yaml path=splat_rule/parameters/feast -->

```yaml
appears_on: surface
space:
  unoccupied: true
  cube_side_feet: 10
  adjacent_to_caster: true
consume_seconds: 3600
disappears_after_seconds: 3600
maximum_participants: 12
```

#### Benefits

<!-- splat:yaml path=splat_rule/parameters/benefits -->

```yaml
duration_seconds: 86400
damage_resistance: poison
condition_immunities:
- frightened
- poisoned
hit_point_maximum_increase:
  dice: 2d10
hit_points_gained: same_as_maximum_increase
```
