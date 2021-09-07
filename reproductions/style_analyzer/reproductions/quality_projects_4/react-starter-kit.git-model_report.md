# Model report for file:///tmp/top-repos-quality-repos-jq9lowhm/react-starter-kit.git HEAD b821a03895ff6901722ed6f4be100699cc72a674

### Dump

```json
{'created_at': '2021-09-02 16:45:55',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': '31226426-18b8-4b5d-aae2-49e8ad0589f7',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-jq9lowhm/react-starter-kit.git b821a03895ff6901722ed6f4be100699cc72a674

# javascript
7 rules, avg.len. 6.1
## train
PPCR: 0.641913
### report
macro
{'f1-score': 0.4468906907001325,
 'precision': 0.4522905467137191,
 'recall': 0.44502229591526943,
 'support': 5247}
micro
{'f1-score': 0.9130931961120641,
 'precision': 0.9130931961120641,
 'recall': 0.9130931961120641,
 'support': 5247}
weighted
{'f1-score': 0.8945716791361972,
 'precision': 0.8813301973372272,
 'recall': 0.9130931961120641,
 'support': 5247}
### report_full
macro
{'f1-score': 0.32754048448170375,
 'precision': 0.4522905467137191,
 'recall': 0.28311497528127516,
 'support': 8174}
micro
{'f1-score': 0.7139557410029059,
 'precision': 0.9130931961120641,
 'recall': 0.5861267433325177,
 'support': 8174}
weighted
{'f1-score': 0.6509806778434357,
 'precision': 0.80820686338113,
 'recall': 0.5861267433325177,
 'support': 8174}
## test
PPCR: 0.640266
### report
macro
{'f1-score': 0.4537849698334423,
 'precision': 0.4672191154899163,
 'recall': 0.4445134667592295,
 'support': 1059}
micro
{'f1-score': 0.9112370160528801,
 'precision': 0.9112370160528801,
 'recall': 0.9112370160528801,
 'support': 1059}
weighted
{'f1-score': 0.8958034215377859,
 'precision': 0.8876629036192287,
 'recall': 0.9112370160528801,
 'support': 1059}
### report_full
macro
{'f1-score': 0.3383553479528417,
 'precision': 0.4672191154899163,
 'recall': 0.2865302689297682,
 'support': 1654}
micro
{'f1-score': 0.7113896056026539,
 'precision': 0.9112370160528801,
 'recall': 0.5834340991535671,
 'support': 1654}
weighted
{'f1-score': 0.6485074913618524,
 'precision': 0.8041337568525753,
 'recall': 0.5834340991535671,
 'support': 1654}
```

## javascript
### Summary
2 rules, avg.len. 6.0

| | |
|-|-|
|Min support|142|
|Max support|182|
|Min confidence|0.9260563254356384|
|Max confidence|0.9972527623176575|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.997. Support: 182.` |
| 2 | `  -1.diff_offset ≥ 5<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 142.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.0, "max_conf": 0.9972527623176575, "max_support": 182, "min_conf": 0.9260563254356384, "min_support": 142, "num_rules": 2}}
```
</details>
