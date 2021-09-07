# Model report for file:///tmp/top-repos-quality-repos-ivbso1fn/hipeerlab-platform-fe.git HEAD 659a5990d2c0f263ed76228a8886bb487ff231e1

### Dump

```json
{'created_at': '2021-08-31 23:36:21',
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
 'size': '17.4 kB',
 'tags': [],
 'uuid': 'd9469cd3-0c71-43b8-9b05-1abaa2d24d75',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ivbso1fn/hipeerlab-platform-fe.git 659a5990d2c0f263ed76228a8886bb487ff231e1

# javascript
46 rules, avg.len. 5.2
## train
PPCR: 0.893747
### report
macro
{'f1-score': 0.5462095755201105,
 'precision': 0.559325447453842,
 'recall': 0.5381731497002175,
 'support': 7461}
micro
{'f1-score': 0.8805790108564535,
 'precision': 0.8805790108564535,
 'recall': 0.8805790108564535,
 'support': 7461}
weighted
{'f1-score': 0.8628902902698324,
 'precision': 0.8505972040562303,
 'recall': 0.8805790108564535,
 'support': 7461}
### report_full
macro
{'f1-score': 0.5118930965975332,
 'precision': 0.559325447453842,
 'recall': 0.4776964819308651,
 'support': 8348}
micro
{'f1-score': 0.8311721171484597,
 'precision': 0.8805790108564535,
 'recall': 0.7870148538572113,
 'support': 8348}
weighted
{'f1-score': 0.8026364736742131,
 'precision': 0.8273294741731203,
 'recall': 0.7870148538572113,
 'support': 8348}
## test
PPCR: 0.874096
### report
macro
{'f1-score': 0.49860221573245106,
 'precision': 0.524478079898211,
 'recall': 0.48234962146305027,
 'support': 1812}
micro
{'f1-score': 0.8537527593818984,
 'precision': 0.8537527593818984,
 'recall': 0.8537527593818984,
 'support': 1812}
weighted
{'f1-score': 0.8402419749537896,
 'precision': 0.8354386689115261,
 'recall': 0.8537527593818984,
 'support': 1812}
### report_full
macro
{'f1-score': 0.464539796811534,
 'precision': 0.524478079898211,
 'recall': 0.4215167255263301,
 'support': 2073}
micro
{'f1-score': 0.7963963963963963,
 'precision': 0.8537527593818984,
 'recall': 0.7462614568258562,
 'support': 2073}
weighted
{'f1-score': 0.7754901850643986,
 'precision': 0.8162036889036035,
 'recall': 0.7462614568258562,
 'support': 2073}
```

## javascript
### Summary
21 rules, avg.len. 3.9

| | |
|-|-|
|Min support|159|
|Max support|2288|
|Min confidence|0.9222028255462646|
|Max confidence|0.9981949329376221|

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
| 1 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = /<br>	∧ +4.length ≥ 6<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 179.` |
| 2 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {/}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.991. Support: 1979.` |
| 3 | `  -1.label in {<space>}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = '<br>Confidence: 0.998. Support: 277.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.955. Support: 232.` |
| 5 | `  -1.internal_type = StringLiteral<br>	∧ +1.length ≤ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = '<br>Confidence: 0.995. Support: 278.` |
| 6 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = /<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 179.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.960. Support: 215.` |
| 8 | `  -1.diff_offset ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ -4.diff_offset ≤ 23<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 483.` |
| 9 | `  -1.diff_offset ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -4.length ≤ 3<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.936. Support: 552.` |
| 10 | `  -1.diff_offset ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.940. Support: 208.` |
| 11 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = /<br>	∧ +4.length ≥ 8<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 159.` |
| 12 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.946. Support: 2288.` |
| 13 | `  -5.roles not in {INCOMPLETE}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.954. Support: 2005.` |
| 14 | `  -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = /<br>	∧ +5.reserved = <<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 15 | `  -1.diff_offset ≥ 3<br>	∧ -1.label not in {<space>}<br>	∧ -4.diff_offset ≤ 23<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 521.` |
| 16 | `  -1.diff_offset ≤ 2<br>	∧ -1.label not in {<space>}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.973. Support: 205.` |
| 17 | `  -1.label not in {<space>}<br>	∧ -1.length ≥ 3<br>	∧ -4.length ≤ 3<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 572.` |
| 18 | `  -1.label not in {<space>}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.length ≥ 3<br>	∧ ^1.internal_type not in {JSXElement}<br>⇒ y = "<br>Confidence: 0.940. Support: 210.` |
| 19 | `  -1.internal_type = StringLiteral<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = "<br>Confidence: 0.922. Support: 161.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = /<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.997. Support: 198.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ ^1.internal_type = JSXElement<br>⇒ y = ∅<br>Confidence: 0.990. Support: 1883.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 3.9047619047619047, "max_conf": 0.9981949329376221, "max_support": 2288, "min_conf": 0.9222028255462646, "min_support": 159, "num_rules": 21}}
```
</details>
