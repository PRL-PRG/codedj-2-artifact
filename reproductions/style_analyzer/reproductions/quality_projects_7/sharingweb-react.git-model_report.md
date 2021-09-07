# Model report for file:///tmp/top-repos-quality-repos-5jzssn9v/sharingweb-react.git HEAD 7ca4eb61cad869853b7b72839506e954b82d570a

### Dump

```json
{'created_at': '2021-09-01 01:43:13',
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
 'size': '17.2 kB',
 'tags': [],
 'uuid': '5493785a-53f1-448f-b2e2-972d7ad07f09',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5jzssn9v/sharingweb-react.git 7ca4eb61cad869853b7b72839506e954b82d570a

# javascript
20 rules, avg.len. 7.3
## train
PPCR: 0.885880
### report
macro
{'f1-score': 0.6372309876175791,
 'precision': 0.6676752887816113,
 'recall': 0.6136640988925482,
 'support': 15929}
micro
{'f1-score': 0.946198756984117,
 'precision': 0.946198756984117,
 'recall': 0.946198756984117,
 'support': 15929}
weighted
{'f1-score': 0.9424869130495107,
 'precision': 0.9406951635442999,
 'recall': 0.946198756984117,
 'support': 15929}
### report_full
macro
{'f1-score': 0.5447869965897247,
 'precision': 0.6676752887816113,
 'recall': 0.49472920237242846,
 'support': 17981}
micro
{'f1-score': 0.8889413152462401,
 'precision': 0.946198756984117,
 'recall': 0.8382181191257438,
 'support': 17981}
weighted
{'f1-score': 0.8646682557538617,
 'precision': 0.9182303580607358,
 'recall': 0.8382181191257438,
 'support': 17981}
## test
PPCR: 0.904206
### report
macro
{'f1-score': 0.6361955333348901,
 'precision': 0.6720085377085188,
 'recall': 0.6129279534438534,
 'support': 3719}
micro
{'f1-score': 0.9556332347405216,
 'precision': 0.9556332347405216,
 'recall': 0.9556332347405216,
 'support': 3719}
weighted
{'f1-score': 0.9537133441848826,
 'precision': 0.9541426820516595,
 'recall': 0.9556332347405216,
 'support': 3719}
### report_full
macro
{'f1-score': 0.5419992506191651,
 'precision': 0.6720085377085188,
 'recall': 0.49796383185058474,
 'support': 4113}
micro
{'f1-score': 0.90755873340143,
 'precision': 0.9556332347405216,
 'recall': 0.8640894724045709,
 'support': 4113}
weighted
{'f1-score': 0.8870775550338482,
 'precision': 0.9355097360624293,
 'recall': 0.8640894724045709,
 'support': 4113}
```

## javascript
### Summary
10 rules, avg.len. 6.4

| | |
|-|-|
|Min support|94|
|Max support|5463|
|Min confidence|0.9240740537643433|
|Max confidence|0.9983713626861572|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^2.internal_type not in {File}<br>⇒ y = "<br>Confidence: 0.935. Support: 653.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.955. Support: 168.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>⇒ y = ␣<br>Confidence: 0.993. Support: 352.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 5463.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1750.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 307.` |
| 7 | `  -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 94.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 270.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {=}<br>	∧ +3.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 517.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type not in {CallExpression, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 171.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.4, "max_conf": 0.9983713626861572, "max_support": 5463, "min_conf": 0.9240740537643433, "min_support": 94, "num_rules": 10}}
```
</details>
