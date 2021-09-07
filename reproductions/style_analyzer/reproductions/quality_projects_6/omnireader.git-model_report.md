# Model report for file:///tmp/top-repos-quality-repos-_z6apyu2/omnireader.git HEAD c34d456c433aa4e0dac8b5f8e9f6d4fd6eaa36db

### Dump

```json
{'created_at': '2021-09-01 23:25:21',
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
 'size': '16.8 kB',
 'tags': [],
 'uuid': '47f37d9f-39f3-4516-9ef0-94dd64315d79',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-_z6apyu2/omnireader.git c34d456c433aa4e0dac8b5f8e9f6d4fd6eaa36db

# javascript
9 rules, avg.len. 4.2
## train
PPCR: 0.836479
### report
macro
{'f1-score': 0.6577986316624747,
 'precision': 0.6925134129177429,
 'recall': 0.638866264378927,
 'support': 8466}
micro
{'f1-score': 0.8659343255374439,
 'precision': 0.8659343255374439,
 'recall': 0.8659343255374439,
 'support': 8466}
weighted
{'f1-score': 0.849927508924782,
 'precision': 0.8467464486810807,
 'recall': 0.8659343255374439,
 'support': 8466}
### report_full
macro
{'f1-score': 0.518798038350251,
 'precision': 0.6925134129177429,
 'recall': 0.44660322710550937,
 'support': 10121}
micro
{'f1-score': 0.7888309033195243,
 'precision': 0.8659343255374439,
 'recall': 0.7243355399664064,
 'support': 10121}
weighted
{'f1-score': 0.756041310618151,
 'precision': 0.844858710783398,
 'recall': 0.7243355399664064,
 'support': 10121}
## test
PPCR: 0.874875
### report
macro
{'f1-score': 0.6545512676580284,
 'precision': 0.6790858277325454,
 'recall': 0.6445352064397445,
 'support': 2615}
micro
{'f1-score': 0.8676864244741874,
 'precision': 0.8676864244741874,
 'recall': 0.8676864244741874,
 'support': 2615}
weighted
{'f1-score': 0.8520311271259515,
 'precision': 0.8515612463297212,
 'recall': 0.8676864244741874,
 'support': 2615}
### report_full
macro
{'f1-score': 0.5242773828755158,
 'precision': 0.6790858277325454,
 'recall': 0.45302636419770165,
 'support': 2989}
micro
{'f1-score': 0.8097787294789435,
 'precision': 0.8676864244741874,
 'recall': 0.7591167614586818,
 'support': 2989}
weighted
{'f1-score': 0.7817187910576354,
 'precision': 0.850875303646091,
 'recall': 0.7591167614586818,
 'support': 2989}
```

## javascript
### Summary
6 rules, avg.len. 3.5

| | |
|-|-|
|Min support|98|
|Max support|153|
|Min confidence|0.9217687249183655|
|Max confidence|0.9967319965362549|

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
                     'max_depth': 10,
                     'min_samples_leaf': 95,
                     'min_samples_split': 203,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.label in {<space>}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.922. Support: 147.` |
| 2 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 98.` |
| 3 | `  -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 110.` |
| 4 | `  -1.reserved = ;<br>	∧ +1.reserved not in {import, }}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {FILE}<br>⇒ y = ⏎⏎<br>Confidence: 0.942. Support: 129.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 153.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 143.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.5, "max_conf": 0.9967319965362549, "max_support": 153, "min_conf": 0.9217687249183655, "min_support": 98, "num_rules": 6}}
```
</details>
