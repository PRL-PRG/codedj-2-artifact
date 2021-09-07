# Model report for file:///tmp/top-repos-quality-repos-mhpdq1pj/dubdub-sites.git HEAD f86014e831aec5000fe943db5b48cfde8fcbff53

### Dump

```json
{'created_at': '2021-09-02 07:22:24',
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
 'uuid': '942d42fe-bb1d-4e9f-b440-30d973652c0a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mhpdq1pj/dubdub-sites.git f86014e831aec5000fe943db5b48cfde8fcbff53

# javascript
13 rules, avg.len. 6.7
## train
PPCR: 0.652982
### report
macro
{'f1-score': 0.35028839429189434,
 'precision': 0.3495786232924425,
 'recall': 0.352751809969751,
 'support': 6919}
micro
{'f1-score': 0.9312039312039312,
 'precision': 0.9312039312039312,
 'recall': 0.9312039312039312,
 'support': 6919}
weighted
{'f1-score': 0.9152429671141091,
 'precision': 0.9016233905217608,
 'recall': 0.9312039312039312,
 'support': 6919}
### report_full
macro
{'f1-score': 0.29991195835429657,
 'precision': 0.3495786232924425,
 'recall': 0.2633619562211753,
 'support': 10596}
micro
{'f1-score': 0.7357122466457324,
 'precision': 0.9312039312039312,
 'recall': 0.6080596451491129,
 'support': 10596}
weighted
{'f1-score': 0.6767965122380943,
 'precision': 0.7658330700024357,
 'recall': 0.6080596451491129,
 'support': 10596}
## test
PPCR: 0.663957
### report
macro
{'f1-score': 0.3484914254189674,
 'precision': 0.34519342446171714,
 'recall': 0.3543226472388588,
 'support': 1715}
micro
{'f1-score': 0.9212827988338192,
 'precision': 0.9212827988338192,
 'recall': 0.9212827988338192,
 'support': 1715}
weighted
{'f1-score': 0.9052032171859229,
 'precision': 0.8930086718609793,
 'recall': 0.9212827988338192,
 'support': 1715}
### report_full
macro
{'f1-score': 0.3105323400113099,
 'precision': 0.34519342446171714,
 'recall': 0.2828007357795252,
 'support': 2583}
micro
{'f1-score': 0.7352256863657515,
 'precision': 0.9212827988338192,
 'recall': 0.6116918312040264,
 'support': 2583}
weighted
{'f1-score': 0.6802834522230199,
 'precision': 0.7670275297537982,
 'recall': 0.6116918312040264,
 'support': 2583}
```

## javascript
### Summary
8 rules, avg.len. 6.5

| | |
|-|-|
|Min support|110|
|Max support|1539|
|Min confidence|0.9288498759269714|
|Max confidence|0.998106062412262|

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
                     'min_samples_split': 194,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 0.998. Support: 264.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.998. Support: 205.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.929. Support: 1539.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 995.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = JSXAttribute<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 607.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement, MemberExpression}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 110.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION, MAP}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 232.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION} and not in {MAP}<br>	∧ -2.roles not in {LITERAL}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {JSXAttribute, JSXElement, MemberExpression}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.5, "max_conf": 0.998106062412262, "max_support": 1539, "min_conf": 0.9288498759269714, "min_support": 110, "num_rules": 8}}
```
</details>
