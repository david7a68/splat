---
splat_schema: 1
splat_id: 7a061d25-9ada-50ee-b2b8-5b7ccfbd063c
splat_kind: spell
splat_key: dnd2024:spells/acid_splash
splat_name: Acid Splash
splat_origin:
  key: dnd2024:reference/spell/acid_splash
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
---

# Acid Splash

## Source text

<!-- SRD page 107 -->

Evocation Cantrip (Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous

You create an acidic bubble at a point within range, where it explodes in a 5-foot-radius Sphere. Each creature in that Sphere must succeed on a Dexterity saving throw or take 1d6 Acid damage.

Cantrip Upgrade. The damage increases by 1d6 when you reach levels 5 (2d6), 11 (3d6), and 17 (4d6).

Original layout: [[../../srd-5.2.1/pages/107|page 107]].

## Spell

<!-- splat:values path=splat_spell -->

| Field | Value |
| --- | --- |
| ` level ` | ` 0 ` |
| ` school ` | ` evocation ` |
| ` classes ` | ` ["sorcerer", "wizard"] ` |
| ` components ` | ` ["verbal", "somatic"] ` |
| ` concentration ` | ` false ` |
| ` duration ` | ` Instantaneous ` |
| ` casting_time ` | ` Action ` |
| ` ritual ` | ` false ` |

### Range

<!-- splat:values path=splat_spell/range -->

| Field | Value |
| --- | --- |
| ` feet ` | ` 60 ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:spells/acid_splash ` |
| ` implementation ` | ` dnd2024:spell_acid_splash ` |

### Parameters

#### Targeting

<!-- splat:values path=splat_rule/parameters/targeting -->

| Field | Value |
| --- | --- |
| ` kind ` | ` creatures_in_area ` |
| ` origin ` | ` chosen_point_in_range ` |
| ` shape ` | ` sphere ` |
| ` radius_feet ` | ` 5 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: dexterity
failure:
  damage:
    dice: 1d6
    type: acid
success:
  damage: 0
```

#### Cantrip scaling

<!-- splat:yaml path=splat_rule/parameters/cantrip_scaling -->

```yaml
character_level_dice:
  '1': 1d6
  '5': 2d6
  '11': 3d6
  '17': 4d6
```

#### Spell lifetime

<!-- splat:yaml path=splat_rule/parameters/spell_lifetime -->

```yaml
concentration: false
maximum:
  kind: instantaneous
```
