# Model report for file:///tmp/top-repos-quality-repos-jf6o__r_/fit-conditions.git HEAD c707db68924d22a738ff5b722d94c31413a86899

### Dump

```json
{'created_at': '2021-09-02 06:17:22',
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
 'size': '17.5 kB',
 'tags': [],
 'uuid': '5fd488af-0f79-4c6d-9b42-3b5abd663106',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jf6o__r_/fit-conditions.git c707db68924d22a738ff5b722d94c31413a86899

# javascript
8 rules, avg.len. 4.4
## train
PPCR: 0.700366
### report
macro
{'f1-score': 0.4033637409471403,
 'precision': 0.4191959438167378,
 'recall': 0.3963350826731726,
 'support': 4406}
micro
{'f1-score': 0.9346345891965502,
 'precision': 0.9346345891965502,
 'recall': 0.9346345891965502,
 'support': 4406}
weighted
{'f1-score': 0.9172898384148767,
 'precision': 0.9078652939272147,
 'recall': 0.9346345891965502,
 'support': 4406}
### report_full
macro
{'f1-score': 0.3221527438873483,
 'precision': 0.4191959438167378,
 'recall': 0.28750574682434493,
 'support': 6291}
micro
{'f1-score': 0.7699354959334394,
 'precision': 0.9346345891965502,
 'recall': 0.6545859163884915,
 'support': 6291}
weighted
{'f1-score': 0.7168896193004938,
 'precision': 0.8665357357683442,
 'recall': 0.6545859163884915,
 'support': 6291}
## test
PPCR: 0.753268
### report
macro
{'f1-score': 0.4065816567020922,
 'precision': 0.41512105868291044,
 'recall': 0.40612915404001754,
 'support': 922}
micro
{'f1-score': 0.9490238611713666,
 'precision': 0.9490238611713666,
 'recall': 0.9490238611713666,
 'support': 922}
weighted
{'f1-score': 0.9400251217691279,
 'precision': 0.9385417849902661,
 'recall': 0.9490238611713666,
 'support': 922}
### report_full
macro
{'f1-score': 0.2653986790398529,
 'precision': 0.41512105868291044,
 'recall': 0.2364685828861458,
 'support': 1224}
micro
{'f1-score': 0.8154706430568499,
 'precision': 0.9490238611713666,
 'recall': 0.7148692810457516,
 'support': 1224}
weighted
{'f1-score': 0.772724775624427,
 'precision': 0.9128377325576772,
 'recall': 0.7148692810457516,
 'support': 1224}
```

## javascript
### Summary
5 rules, avg.len. 4.0

| | |
|-|-|
|Min support|125|
|Max support|996|
|Min confidence|0.9320651888847351|
|Max confidence|0.9967948794364929|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 96,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 156.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.977. Support: 996.` |
| 3 | `  -1.internal_type = CommentLine<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ⏎<br>Confidence: 0.972. Support: 125.` |
| 4 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 461.` |
| 5 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 184.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.0, "max_conf": 0.9967948794364929, "max_support": 996, "min_conf": 0.9320651888847351, "min_support": 125, "num_rules": 5}}
```
</details>
