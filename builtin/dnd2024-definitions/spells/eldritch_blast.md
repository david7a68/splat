---
splat_schema: 1
splat_id: 955fbc1e-8225-57c6-89f5-0c4bdc89b22a
splat_kind: spell
splat_key: dnd2024:spells/eldritch_blast
splat_name: Eldritch Blast
splat_origin:
  key: dnd2024:reference/spell/eldritch_blast
splat_sources:
- title: SRD 5.2.1
  locator: pp. 127
  url: https://www.dndbeyond.com/srd
---

# Eldritch Blast

## Source text

<!-- SRD page 127 -->

Evocation Cantrip (Warlock)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: Instantaneous

You hurl a beam of crackling energy. Make a ranged spell attack against one creature or object in range. On a hit, the target takes 1d10 Force damage.

Cantrip Upgrade. The spell creates two beams at level 5, three beams at level 11, and four beams at level 17. You can direct the beams at the same target or at different ones. Make a separate attack roll for each beam.

Original layout: [[../../srd-5.2.1/pages/127|page 127]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["warlock"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 120 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/eldritch_blast ` |
| ` implementation ` | ` dnd2024:spell_eldritch_blast ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creature_or_object ` |
| ` within_range ` | ` true ` |
| ` per_beam ` | ` true ` |
| ` same_or_different ` | ` true ` |

#### Beams

<!-- splat:yaml path=splat_rule/parameters/beams -->

```yaml
character_level_count:
  '1': 1
  '5': 2
  '11': 3
  '17': 4
independent_attacks: true
```

#### Attack

<!-- splat:yaml path=splat_rule/parameters/attack -->

```yaml
kind: ranged_spell
hit:
  damage:
    dice: 1d10
    type: force
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
