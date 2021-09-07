# Model report for file:///tmp/top-repos-quality-repos-ar1ujtb6/hr-service-desk-alpha.git HEAD 784cd966b08dfff7c4f97bec6bdb43338255af43

### Dump

```json
{'created_at': '2021-08-31 23:55:34',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.1 kB',
 'tags': [],
 'uuid': '44d953e5-5db2-426f-8057-78ef7cda76c4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ar1ujtb6/hr-service-desk-alpha.git 784cd966b08dfff7c4f97bec6bdb43338255af43

# javascript
50 rules, avg.len. 4.6
## train
PPCR: 0.871795
### report
macro
{'f1-score': 0.37244287942660154,
 'precision': 0.37524986855477327,
 'recall': 0.37254004385699757,
 'support': 5474}
micro
{'f1-score': 0.8498355864084765,
 'precision': 0.8498355864084765,
 'recall': 0.8498355864084765,
 'support': 5474}
weighted
{'f1-score': 0.822061017560081,
 'precision': 0.7996626982588962,
 'recall': 0.8498355864084765,
 'support': 5474}
### report_full
macro
{'f1-score': 0.3617778357593174,
 'precision': 0.37524986855477327,
 'recall': 0.3513028085053599,
 'support': 6279}
micro
{'f1-score': 0.7916276695311837,
 'precision': 0.8498355864084765,
 'recall': 0.7408823060996974,
 'support': 6279}
weighted
{'f1-score': 0.7410326262357999,
 'precision': 0.74530463613613,
 'recall': 0.7408823060996974,
 'support': 6279}
## test
PPCR: 0.925550
### report
macro
{'f1-score': 0.38562487174022664,
 'precision': 0.40191441441441444,
 'recall': 0.3739278912607574,
 'support': 547}
micro
{'f1-score': 0.9159049360146252,
 'precision': 0.9159049360146252,
 'recall': 0.9159049360146252,
 'support': 547}
weighted
{'f1-score': 0.901580371208434,
 'precision': 0.8923120789894099,
 'recall': 0.9159049360146252,
 'support': 547}
### report_full
macro
{'f1-score': 0.3807261559726073,
 'precision': 0.40191441441441444,
 'recall': 0.3645993311685675,
 'support': 591}
micro
{'f1-score': 0.8804920913884008,
 'precision': 0.9159049360146252,
 'recall': 0.8477157360406091,
 'support': 591}
weighted
{'f1-score': 0.845107235049653,
 'precision': 0.8473578908858097,
 'recall': 0.8477157360406091,
 'support': 591}
```

## javascript
### Summary
26 rules, avg.len. 3.8

| | |
|-|-|
|Min support|132|
|Max support|816|
|Min confidence|0.9276729822158813|
|Max confidence|0.9979674816131592|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 774.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.997. Support: 156.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.965. Support: 156.` |
| 4 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 741.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 167.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {KEY}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 142.` |
| 7 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.970. Support: 816.` |
| 8 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.998. Support: 239.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {KEY}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 135.` |
| 10 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {KEY}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 363.` |
| 11 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.998. Support: 246.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 770.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 162.` |
| 14 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.998. Support: 220.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 153.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.976. Support: 721.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 0.928. Support: 159.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 153.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.945. Support: 137.` |
| 20 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 209.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 557.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 155.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 132.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 736.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 522.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.938. Support: 153.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.8076923076923075, "max_conf": 0.9979674816131592, "max_support": 816, "min_conf": 0.9276729822158813, "min_support": 132, "num_rules": 26}}
```
</details>
