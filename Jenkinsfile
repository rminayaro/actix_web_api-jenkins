pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "localhost:8082"
        DOCKER_IMAGE = "actix_web_api"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "64.23.161.84"
        SERVER_PASSWORD = "Ramon2Minaya" // Contraseña de tu servidor
        GITHUB_CREDENTIALS = "github-credentials"
        GITHUB_REPO = "https://github.com/rminayaro/actix_web_api---Paralelo.git"
        NEXUS_USER = "admin"
        NEXUS_PASSWORD = "123456"
        PLINK_PATH = "\"C:\\Program Files\\PuTTY\\plink.exe\"" // Ruta correcta de plink.exe con comillas dobles
    }
    stages {
        stage('Checkout') {
            steps {
                echo "📥 Clonando código fuente desde GitHub..."
                git branch: 'main', credentialsId: GITHUB_CREDENTIALS, url: GITHUB_REPO
            }
        }
        stage('Build Docker Image') {
            steps {
                echo "🔨 Construyendo imagen Docker..."
                bat "docker build -t ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG} ."
            }
        }
        stage('Login to Nexus') {
            steps {
                echo "🔑 Iniciando sesión en Nexus para su verificación..."
                bat "docker login -u ${NEXUS_USER} -p '${NEXUS_PASSWORD}' ${DOCKER_REGISTRY}"
            }
        }
        stage('Push to Nexus') {
            steps {
                echo "📤 Subiendo imagen a Nexus..."
                bat "docker push ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG}"
            }
        }
        stage('Deploy to Server') {
            steps {
                echo "🚀 Desplegando aplicación en el servidor..."
                script {
                    // Usar plink para conectarse al servidor remoto y ejecutar comandos Docker
                    bat """
                        ${PLINK_PATH} -batch -ssh ${SERVER_USER}@${SERVER_IP} -pw ${SERVER_PASSWORD} -o StrictHostKeyChecking=no ^
                        "docker pull ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG} && ^
                        docker stop ${DOCKER_IMAGE} || true && ^
                        docker rm -f ${DOCKER_IMAGE} || true && ^
                        docker run -d --restart unless-stopped --name ${DOCKER_IMAGE} -p 3030:3030 ${DOCKER_REGISTRY}/${DOCKER_IMAGE}:${DOCKER_TAG}"
                    """
                }
            }
        }
    }
    post {
        success {
            echo "🎉 Despliegue exitoso de la aplicación!"
        }
        failure {
            echo "🚨 ERROR en el despliegue!"
        }
    }
}
