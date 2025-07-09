import streamlit as st
import os
import requests
import json
import plotly.graph_objects as go
import plotly.express as px
import pandas as pd
from PIL import Image

# API_BASE = "https://10d5-2-36-96-217.ngrok-free.app" # NGROK URL
# API_BASE = "http://localhost:8080" # LOCAL URL
API_BASE = "http://3.95.132.214:80"  # Your EC2 API

st.title('House Price Predictor for Boston Housing Dataset (1970s)')

# Add image below the title


# Create input fields for all features 
st.subheader('Tech Stack: Rust, Python, Docker, XGBoost, Streamlit, AWS S3, AWS EC2')
st.markdown("---")
st.markdown("##### Select the feature values to predict the price of a house in Boston.")
st.markdown("This is the frontend for the house price predictor inferences service. The backend is a Rust executable binary with an accessible RESt API running in an EC2 instance. The model has been trained also with rust and saved in an S3 bucket.")
st.markdown("---")
col1, col2 = st.columns(2)

with col1:
    crim = st.slider('Crime rate', min_value=0.00632, max_value=88.9762, value=3.613524, step=0.1)
    zn = st.slider('Proportion of residential land zoned', min_value=0.0, max_value=100.0, value=11.363636, step=1.0)
    indus = st.slider('Proportion of non-retail business acres', min_value=0.46, max_value=27.74, value=11.136779, step=0.1)
    chas = st.selectbox('Charles River dummy variable', options=[0.0, 1.0], index=0)
    nox = st.slider('Nitric oxides concentration', min_value=0.385, max_value=0.871, value=0.554695, step=0.001)
    rm = st.slider('number of rooms', min_value=3.561, max_value=8.78, value=6.284634, step=0.1)
    
with col2:
    age = st.slider('Proportion of owner-occupied units built prior to 1940', min_value=2.9, max_value=100.0, value=68.574901, step=1.0)
    dis = st.slider('Weighted distances to employment centres', min_value=1.1296, max_value=12.1265, value=3.795043, step=0.1)
    rad = st.slider('Index of accessibility to radial highways', min_value=1.0, max_value=24.0, value=9.549407, step=1.0)
    tax = st.slider('Property-tax rate', min_value=187.0, max_value=711.0, value=408.237154, step=1.0)
    ptratio = st.slider('Pupil-teacher ratio', min_value=12.6, max_value=22.0, value=18.455534, step=0.1)
    rac = st.slider("Racial composition index (legacy feature)", min_value=0.0, max_value=1.0, value=0.0, step=0.01)

    lstat = st.slider('% lower status of the population', min_value=1.73, max_value=37.97, value=12.653063, step=0.01)

# Add sidebar note about dataset bias

script_dir = os.path.dirname(os.path.abspath(__file__))
image_path = os.path.join(script_dir, 'assets', 'boston_housing.png')
image = Image.open(image_path)

st.sidebar.image(image)
st.sidebar.markdown("---")
st.sidebar.markdown("### ⚠️ Note on Dataset Bias")

st.sidebar.info(
    "THE BOSTON HOUSING DATASET includes a legacy feature related to racial demographics "
    "that is widely recognized as ethically problematic. It has been retained "
    "for transparency, but please interpret it with caution. In real-world applications, "
    "this feature would typically be excluded or used only to study and mitigate bias. "
    "The data was originally published by Harrison, D. and Rubinfeld, D.L. `Hedonic prices and the demand for clean air', J. Environ. Economics & Management, vol.5, 81-102, 1978."
)
st.sidebar.markdown("---")
# Add development context note
st.sidebar.markdown("### 🦀 Development Context")
st.sidebar.info(
    "Note: The core funtionality of the application was developed as part of the \"Let's Rust!\" "
    "cohort led by Pau Labarta Bajo, focusing on building real-world machine "
    "learning systems using Rust."
)

# Create a radar chart for key metrics
def create_radar_chart(input_values):
    categories = ['Rooms', 'Crime Rate', 'Age', 'Distance', 'Tax Rate', 'School Ratio']
    
    # Normalize values using actual data ranges from the dataset
    values = [
        (input_values['rm'] - 3.561) / (8.78 - 3.561),  # RM: min=3.561, max=8.78
        (input_values['crim'] - 0.00632) / (88.9762 - 0.00632),  # CRIM: min=0.00632, max=88.9762
        (input_values['age'] - 2.9) / (100.0 - 2.9),  # AGE: min=2.9, max=100.0
        (input_values['dis'] - 1.1296) / (12.1265 - 1.1296),  # DIS: min=1.1296, max=12.1265
        (input_values['tax'] - 187.0) / (711.0 - 187.0),  # TAX: min=187.0, max=711.0
        (input_values['ptratio'] - 12.6) / (22.0 - 12.6)  # PTRATIO: min=12.6, max=22.0
    ]
    
    # Ensure values are within [0, 1] range
    values = [max(0, min(1, v)) for v in values]
    
    fig = go.Figure()
    fig.add_trace(go.Scatterpolar(
        r=values,
        theta=categories,
        fill='toself',
        name='House Features'
    ))
    
    fig.update_layout(
        polar=dict(
            radialaxis=dict(
                visible=True,
                range=[0, 1]
            )),
        showlegend=False,
        title='Property Feature Overview'
    )
    return fig

if st.button('Predict Price'):
    # Prepare the input data
    input_data = {
        "crim": crim,
        "zn": zn,
        "indus": indus,
        "chas": chas,
        "nox": nox,
        "rm": rm,
        "age": age,
        "dis": dis,
        "rad": rad,
        "tax": tax,
        "ptratio": ptratio,
        "b": rac,
        "lstat": lstat
    }
    
    try:
        # Add session with retry strategy
        session = requests.Session()
        adapter = requests.adapters.HTTPAdapter(
            max_retries=3,
            pool_connections=1,
            pool_maxsize=1
        )
        session.mount('http://', adapter)
        
        response = session.post(
            f"{API_BASE}/predict",
            headers={"Content-Type": "application/json"},
            data=json.dumps(input_data),
            timeout=10,
            verify=False
        )
        
        if response.status_code == 200:
            st.success("✅ API connection successful!")
            prediction = response.json()["prediction"]
            
            # Create columns for layout
            col1, col2, col3 = st.columns([1,3,1])
            
            with col2:
                # Display price in smaller font with markdown and darker color
                st.markdown(f"<h3 style='text-align: center; color: #1e3a8a;'>Predicted Price: ${prediction:,.2f}k</h3>", unsafe_allow_html=True)
            
            # Display radar chart of input features below the price
            st.plotly_chart(create_radar_chart(input_data))
            
        else:
            st.error(f'Error getting prediction from API. Status code: {response.status_code}')
            
    except requests.exceptions.ConnectionError as e:
        st.error(f'Could not connect to the API. Error: {str(e)}')
        st.info('Please check:\n1. Is the API server running?\n2. Is port 8080 exposed in Docker?\n3. Is the URL correct?') 