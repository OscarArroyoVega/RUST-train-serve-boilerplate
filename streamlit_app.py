import streamlit as st
import requests
import json
import plotly.graph_objects as go
import plotly.express as px
import pandas as pd
from PIL import Image

# API_BASE = "https://10d5-2-36-96-217.ngrok-free.app" # NGROK URL
# API_BASE = "http://localhost:8080" # LOCAL URL
API_BASE = "http://3.95.132.214:80"  # Your EC2 API

st.title('House Price Predictor for Boston Housing Dataset using XGBoost in Rust')

# Add image below the title


# Create input fields for all features 
st.subheader('Tech Stack: Rust, Python, Docker, XGBoost, Streamlit, AWS S3, AWS EC2')
st.markdown("---")
st.markdown("##### Select the feature values to predict the price of a house in Boston.")
st.markdown("This is the frontend for the house price predictor inferences service. The backend is a Rust executable binary with an accessible RESt API running in an EC2 instance. The model has been trained also with rust and saved in an S3 bucket.")
st.markdown("---")
col1, col2 = st.columns(2)

with col1:
    crim = st.slider('Crime rate', min_value=0.0, max_value=100.0, value=0.00632, step=0.01)
    zn = st.slider('Proportion of residential land zoned', min_value=0.0, max_value=100.0, value=18.0, step=1.0)
    indus = st.slider('Proportion of non-retail business acres', min_value=0.0, max_value=30.0, value=2.31, step=0.01)
    chas = st.selectbox('Charles River dummy variable', options=[0.0, 1.0], index=0)
    nox = st.slider('Nitric oxides concentration', min_value=0.0, max_value=1.0, value=0.538, step=0.001)
    rm = st.slider('number of rooms', min_value=3.0, max_value=9.0, value=6.575, step=0.1)
    
with col2:
    age = st.slider('Proportion of owner-occupied units built prior to 1940', min_value=0.0, max_value=100.0, value=65.2, step=1.0)
    dis = st.slider('Weighted distances to employment centres', min_value=1.0, max_value=12.0, value=4.0900, step=0.1)
    rad = st.slider('Index of accessibility to radial highways', min_value=1.0, max_value=24.0, value=1.0, step=1.0)
    tax = st.slider('Property-tax rate', min_value=100.0, max_value=800.0, value=296.0, step=1.0)
    ptratio = st.slider('Pupil-teacher ratio', min_value=12.0, max_value=22.0, value=15.3, step=0.1)
    rac = st.slider("Racial composition index (legacy feature)", min_value=0.0, max_value=1.0, step=0.01)

    lstat = st.slider('% lower status of the population', min_value=1.0, max_value=40.0, value=4.98, step=0.01)

# Add sidebar note about dataset bias

image = Image.open('boston_housing.png')

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
    "Note: This application is being developed as part of the \"Let's Rust!\" "
    "cohort led by Pau Labarta Bajo, focusing on building real-world machine "
    "learning systems using Rust."
)

# Create a radar chart for key metrics
def create_radar_chart(input_values):
    categories = ['Rooms', 'Crime Rate', 'Age', 'Distance', 'Tax Rate', 'School Ratio']
    values = [
        input_values['rm'] / 9,  # Normalize to 0-1
        input_values['crim'] / 100,  # Normalize to 0-1
        input_values['age'] / 100,  # Normalize to 0-1
        input_values['dis'] / 12,   # Normalize to 0-1
        input_values['tax'] / 800,  # Normalize to 0-1
        input_values['ptratio'] / 22 # Normalize to 0-1
    ]
    
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