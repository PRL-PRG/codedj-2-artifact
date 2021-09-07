# Model report for file:///tmp/top-repos-quality-repos-8_pugp2s/matt-peet-illustration.git HEAD aab58c9d78342452aac4819e0b72474ce8b45e0a

### Dump

```json
{'created_at': '2021-09-02 01:22:02',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-74-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.1 kB',
 'tags': [],
 'uuid': '72efd287-0b6b-4a6d-a55a-20a0db21f454',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-8_pugp2s/matt-peet-illustration.git aab58c9d78342452aac4819e0b72474ce8b45e0a

# javascript
17 rules, avg.len. 5.6
## train
PPCR: 0.686279
### report
macro
{'f1-score': 0.1258161523665032,
 'precision': 0.11240741637217408,
 'recall': 0.14285714285714285,
 'support': 2951}
micro
{'f1-score': 0.7868519146052185,
 'precision': 0.7868519146052185,
 'recall': 0.7868519146052185,
 'support': 2951}
weighted
{'f1-score': 0.6929907626449147,
 'precision': 0.6191359355178981,
 'recall': 0.7868519146052185,
 'support': 2951}
### report_full
macro
{'f1-score': 0.12265272165438555,
 'precision': 0.11240741637217408,
 'recall': 0.13495292339881437,
 'support': 4300}
micro
{'f1-score': 0.6404633843607778,
 'precision': 0.7868519146052185,
 'recall': 0.54,
 'support': 4300}
weighted
{'f1-score': 0.49078202995008324,
 'precision': 0.4497865130464249,
 'recall': 0.54,
 'support': 4300}
## test
PPCR: 0.542662
### report
macro
{'f1-score': 0.12111801242236023,
 'precision': 0.10512129380053907,
 'recall': 0.14285714285714285,
 'support': 159}
micro
{'f1-score': 0.7358490566037735,
 'precision': 0.7358490566037735,
 'recall': 0.7358490566037735,
 'support': 159}
weighted
{'f1-score': 0.6238720262510253,
 'precision': 0.5414738341046635,
 'recall': 0.7358490566037735,
 'support': 159}
### report_full
macro
{'f1-score': 0.11448140900195694,
 'precision': 0.10512129380053907,
 'recall': 0.12567132116004295,
 'support': 293}
micro
{'f1-score': 0.5176991150442478,
 'precision': 0.7358490566037735,
 'recall': 0.3993174061433447,
 'support': 293}
weighted
{'f1-score': 0.36376174669222494,
 'precision': 0.33402022023311223,
 'recall': 0.3993174061433447,
 'support': 293}
```

## javascript
### Summary
7 rules, avg.len. 5.4

| | |
|-|-|
|Min support|234|
|Max support|294|
|Min confidence|0.9720149040222168|
|Max confidence|0.9982993006706238|

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
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 185,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 234.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 273.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 259.` |
| 4 | `  ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 294.` |
| 5 | `  +1.length ≤ 1<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.internal_type = JSXOpeningElement<br>⇒ y = ∅<br>Confidence: 0.972. Support: 268.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION, INCOMPLETE}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 262.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 271.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.428571428571429, "max_conf": 0.9982993006706238, "max_support": 294, "min_conf": 0.9720149040222168, "min_support": 234, "num_rules": 7}}
```
</details>
