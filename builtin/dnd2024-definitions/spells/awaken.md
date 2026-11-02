---
splat_schema: 1
splat_id: 62374024-138f-53e9-ac03-6d7e614f524c
splat_kind: spell
splat_key: dnd2024:spells/awaken
splat_name: Awaken
splat_origin:
  key: dnd2024:reference/spell/awaken
splat_sources:
- title: SRD 5.2.1
  locator: pp. 111, 112
  url: https://www.dndbeyond.com/srd
---

# Awaken

## Source text

<!-- SRD page 111 -->

Level 5 Transmutation (Bard, Druid)\
Casting Time: 8 hours\
Range: Touch\
Components: V, S, M (an agate worth 1,000+ GP, which the spell consumes)\
Duration: Instantaneous

You spend the casting time tracing magical pathways within a precious gemstone, and then touch the target. The target must be either a Beast or Plant creature with an Intelligence of 3 or less or a natural plant that isn’t a creature. The target gains an Intelligence of 10 and the ability to speak one language you know. If the target is a natural plant, it becomes a Plant creature and gains the ability to move its limbs, roots, vines, creepers, and so forth, and it gains senses similar to a human’s. The GM chooses statistics appropriate for the awakened Plant, such as the statistics for the Awakened Shrub or Awakened Tree in “Monsters.”

<!-- SRD page 112 -->

The awakened target has the Charmed condition for 30 days or until you or your allies deal damage to it. When that condition ends, the awakened creature chooses its attitude toward you.

Original layout: [[../../srd-5.2.1/pages/111|page 111]], [[../../srd-5.2.1/pages/112|page 112]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 5 ` |
| ` school ` | ` transmutation ` |
| ` classes ` | ` ["bard", "druid"] ` |
| ` components ` | ` ["verbal", "somatic", "material"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` 8 hours ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` mode ` | ` touch ` |

### Material

<!-- splat:values path=splat_spell/material -->

| Field | Value |
| --- | --- |
| ` description ` | ` an agate worth 1,000+ GP, which the spell consumes ` |
| ` consumed ` | ` true ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/awaken ` |
| ` implementation ` | ` dnd2024:spell_awaken ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` touch ` | ` true ` |

##### Options

<!-- splat:table
path: splat_rule/parameters/targeting/options
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Types:
    path: types
    type: yaml
    optional: true
  Intelligence max:
    path: intelligence_max
    type: integer
    optional: true
  Creature:
    path: creature
    type: boolean
    optional: true
-->

| Key | Kind | Types | Intelligence max | Creature |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` Creature ` | ` ["beast", "plant"] ` | ` 3 ` |  |
| ` row_2 ` | ` Natural plant ` |  |  | false |

#### Grant

<!-- splat:yaml path=splat_rule/parameters/grant -->

```yaml
intelligence: 10
speaks_language:
  choice: caster_known_language
  count: 1
```

#### Natural plant

<!-- splat:yaml path=splat_rule/parameters/natural_plant -->

```yaml
becomes_type: plant
movable_parts:
- limbs
- roots
- vines
- creepers
senses: human_equivalent
statistics:
  chooser: gm
  type: definition
  constraint: appropriate_awakened_plant
```

#### Charm

<!-- splat:yaml path=splat_rule/parameters/charm -->

```yaml
condition: charmed
source: caster
days: 30
ends_on_damage_by:
- caster
- caster_allies
after_end_attitude:
  chooser: awakened_creature
  type: choice
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
