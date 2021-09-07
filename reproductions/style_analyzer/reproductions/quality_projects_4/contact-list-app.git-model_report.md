# Model report for file:///tmp/top-repos-quality-repos-0vofpx0m/contact-list-app.git HEAD d2d9c26d20f6e8242f8ada0803628a01ea89e263

### Dump

```json
{'created_at': '2021-09-02 08:39:01',
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
 'size': '13.4 kB',
 'tags': [],
 'uuid': '7e9c88b3-a2cd-41e2-8924-4d85aa6b59be',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-0vofpx0m/contact-list-app.git d2d9c26d20f6e8242f8ada0803628a01ea89e263

# javascript
7 rules, avg.len. 1.4
## train
PPCR: 0.290698
### report
macro
{'f1-score': 0.19571984435797665,
 'precision': 0.19161904761904763,
 'recall': 0.2,
 'support': 525}
micro
{'f1-score': 0.9580952380952381,
 'precision': 0.9580952380952381,
 'recall': 0.9580952380952381,
 'support': 525}
weighted
{'f1-score': 0.9375912544005929,
 'precision': 0.917946485260771,
 'recall': 0.9580952380952381,
 'support': 525}
### report_full
macro
{'f1-score': 0.12905708787684414,
 'precision': 0.19161904761904763,
 'recall': 0.09729206963249516,
 'support': 1806}
micro
{'f1-score': 0.4315744315744316,
 'precision': 0.9580952380952381,
 'recall': 0.27851605758582504,
 'support': 1806}
weighted
{'f1-score': 0.36944913860646966,
 'precision': 0.5485440067499868,
 'recall': 0.27851605758582504,
 'support': 1806}
## test
PPCR: 0.304878
### report
macro
{'f1-score': 0.18902953586497892,
 'precision': 0.1792,
 'recall': 0.2,
 'support': 125}
micro
{'f1-score': 0.8960000000000001,
 'precision': 0.896,
 'recall': 0.896,
 'support': 125}
weighted
{'f1-score': 0.8468523206751056,
 'precision': 0.8028160000000001,
 'recall': 0.896,
 'support': 125}
### report_full
macro
{'f1-score': 0.12691218130311613,
 'precision': 0.1792,
 'recall': 0.09824561403508772,
 'support': 410}
micro
{'f1-score': 0.4186915887850467,
 'precision': 0.896,
 'recall': 0.2731707317073171,
 'support': 410}
weighted
{'f1-score': 0.3528777724037863,
 'precision': 0.49826341463414636,
 'recall': 0.2731707317073171,
 'support': 410}
```

## javascript
### Summary
6 rules, avg.len. 1.5

| | |
|-|-|
|Min support|151|
|Max support|286|
|Min confidence|0.9370861053466797|
|Max confidence|0.9771126508712769|

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
                     'max_depth': 5,
                     'min_samples_leaf': 90,
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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.967. Support: 286.` |
| 2 | `  ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 246.` |
| 3 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 284.` |
| 4 | `  ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.937. Support: 151.` |
| 5 | `  ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 278.` |
| 6 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 268.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 1.5, "max_conf": 0.9771126508712769, "max_support": 286, "min_conf": 0.9370861053466797, "min_support": 151, "num_rules": 6}}
```
</details>
