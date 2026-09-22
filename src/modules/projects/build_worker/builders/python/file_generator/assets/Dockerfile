FROM python:3.12-slim
WORKDIR /app

ENV PYTHONUNBUFFERED=1
ENV PORT=8000

COPY requirements.txt* pyproject.toml* ./
RUN if [ -f requirements.txt ]; then pip install --no-cache-dir -r requirements.txt; fi

COPY . .

EXPOSE 8000
CMD ["sh", "-c", "if [ -f main.py ]; then python main.py; elif [ -f app.py ]; then python app.py; else python -m http.server 8000; fi"]
