# Model report for file:///tmp/top-repos-quality-repos-o55xlgl9/gatsby-starter-netlify-cms.git HEAD 5c5addbb565f1aae3c1ea15a37b17f42eb65ba6a

### Dump

```json
{'created_at': '2021-09-01 01:01:42',
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
 'size': '16.4 kB',
 'tags': [],
 'uuid': 'ec6d4048-d509-4bf3-aa33-bc59259ddd38',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-o55xlgl9/gatsby-starter-netlify-cms.git 5c5addbb565f1aae3c1ea15a37b17f42eb65ba6a

# javascript
8 rules, avg.len. 4.8
## train
PPCR: 0.795913
### report
macro
{'f1-score': 0.4644469964506642,
 'precision': 0.47182882515038194,
 'recall': 0.45985049856520327,
 'support': 6115}
micro
{'f1-score': 0.9201962387571544,
 'precision': 0.9201962387571545,
 'recall': 0.9201962387571545,
 'support': 6115}
weighted
{'f1-score': 0.9024181249264096,
 'precision': 0.8892801658140943,
 'recall': 0.9201962387571545,
 'support': 6115}
### report_full
macro
{'f1-score': 0.4203439319866711,
 'precision': 0.47182882515038194,
 'recall': 0.3948919782474184,
 'support': 7683}
micro
{'f1-score': 0.8156254529641978,
 'precision': 0.9201962387571545,
 'recall': 0.7323961994012755,
 'support': 7683}
weighted
{'f1-score': 0.7586012901266388,
 'precision': 0.823564255782214,
 'recall': 0.7323961994012755,
 'support': 7683}
## test
PPCR: 0.809555
### report
macro
{'f1-score': 0.46721820134153313,
 'precision': 0.46978566605852523,
 'recall': 0.46526335084932174,
 'support': 1237}
micro
{'f1-score': 0.931285367825384,
 'precision': 0.931285367825384,
 'recall': 0.931285367825384,
 'support': 1237}
weighted
{'f1-score': 0.9208420095171999,
 'precision': 0.9116049650995628,
 'recall': 0.931285367825384,
 'support': 1237}
### report_full
macro
{'f1-score': 0.43446792593722344,
 'precision': 0.46978566605852523,
 'recall': 0.41451369409381333,
 'support': 1528}
micro
{'f1-score': 0.8332730560578662,
 'precision': 0.931285367825384,
 'recall': 0.7539267015706806,
 'support': 1528}
weighted
{'f1-score': 0.7829743296501408,
 'precision': 0.8350785716375541,
 'recall': 0.7539267015706806,
 'support': 1528}
```

## javascript
### Summary
4 rules, avg.len. 2.8

| | |
|-|-|
|Min support|146|
|Max support|201|
|Min confidence|0.9965753555297852|
|Max confidence|0.9975124597549438|

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
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.997. Support: 180.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 0.998. Support: 201.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.997. Support: 164.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.997. Support: 146.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 2.75, "max_conf": 0.9975124597549438, "max_support": 201, "min_conf": 0.9965753555297852, "min_support": 146, "num_rules": 4}}
```
</details>
