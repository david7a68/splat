---
splat_schema: 1
splat_id: 90f2f9db-29eb-5ade-a417-c59abc81984c
splat_kind: rule_group
splat_key: dnd2024:species/human
splat_name: Human
splat_origin:
  key: dnd2024:reference/pages/086
splat_sources:
- title: SRD 5.2.1
  locator: pp. 86
  url: https://www.dndbeyond.com/srd
---

# Human

## Source text

<!-- SRD page 86 -->

Creature Type: Humanoid\
Size: Medium (about 4–7 feet tall) or Small (about 2–4\
feet tall), chosen when you select this species\
Speed: 30 feet\
As a Human, you have these special traits.\
Resourceful. You gain Heroic Inspiration when-\
ever you finish a Long Rest.\
Skillful. You gain proficiency in one skill of your\
choice.\
Versatile. You gain an Origin feat of your choice\
(see “Feats”). Skilled is recommended.\
Fiendish Legacies\
Legacy Level 1\
Abyssal You have Resistance to Poison damage.\
You also know the Poison Spray cantrip.\
Chthonic You have Resistance to Necrotic damage.\
You also know the Chill Touch cantrip.\
Infernal You have Resistance to Fire damage.\
You also know the Fire Bolt cantrip.\

## Rule group

<!-- splat:values path=splat_rule_group -->

| Field | Value |
| --- | --- |
| ` category ` | ` species ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:species/human ` |
| ` implementation ` | ` dnd2024:species_human ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` creature_type ` | ` humanoid ` |
| ` speed_feet ` | ` 30 ` |

#### Size choice

<!-- splat:yaml path=splat_rule/parameters/size_choice -->

```yaml
count: 1
options:
- size: medium
  height_feet_approximate:
    minimum: 4
    maximum: 7
- size: small
  height_feet_approximate:
    minimum: 2
    maximum: 4
```

#### Resourceful

<!-- splat:values path=splat_rule/parameters/resourceful -->

| Field | Value |
| --- | --- |
| ` event ` | ` rest.long_rest_completed ` |
| ` grant ` | ` heroic_inspiration ` |

#### Skillful

<!-- splat:yaml path=splat_rule/parameters/skillful -->

```yaml
proficiency_choice:
  count: 1
  category: any_skill
```

#### Versatile

<!-- splat:yaml path=splat_rule/parameters/versatile -->

```yaml
feat_choice:
  count: 1
  category: origin
```
